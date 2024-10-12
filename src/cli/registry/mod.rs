pub mod errors;
pub mod options;
pub mod version_target;

use anyhow::Result;
use errors::PackageError;
use options::{RegistryClientOptions, RegistryOptions};
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT},
  Client, ClientBuilder,
};
use serde::{Deserialize, Serialize};
use url::Url;
use version_target::VersionTarget;

use super::{flags::Flags, package_info::PackageInfo, updater::can_update};

#[derive(Deserialize, Serialize, Debug)]
struct VersionData {
  version: String,
  deprecated: Option<bool>,
  time: Option<String>,
}

#[derive(Debug)]
pub struct RegistryClient {
  pub client: Client,
  pub registry_url: String,
}

impl Default for RegistryClient {
  fn default() -> Self {
    let registry_options = RegistryClientOptions::default();

    let client = ClientBuilder::new()
      .pool_max_idle_per_host(registry_options.max_sockets)
      .timeout(std::time::Duration::from_secs(registry_options.timeout))
      .danger_accept_invalid_certs(!registry_options.strict_ssl)
      .build()
      .unwrap();

    Self {
      client,
      registry_url: "https://registry.npmjs.org".to_string(),
    }
  }
}

impl RegistryClient {
  pub async fn get_package_info(
    &self,
    name: &str,
    current_version: &str,
    flags: &Flags,
  ) -> Result<Option<PackageInfo>> {
    let latest_version = self.fetch_package_version(name, flags).await?;

    if can_update(current_version, &latest_version)? {
      Ok(Some(PackageInfo {
        pkg_name: name.to_string(),
        current_version: current_version.to_string(),
        latest_version: latest_version.to_string(),
      }))
    } else {
      Ok(None)
    }
  }

  async fn fetch_package_version(&self, name: &str, flags: &Flags) -> Result<String, PackageError> {
    let options = RegistryOptions {
      target: flags.target.clone(),
      registry_url: flags.registry_url.clone(),
      ..Default::default()
    };

    let version_data = self.fetch_registry(name, options).await?;

    Ok(version_data.version)
  }

  async fn fetch_registry(
    &self,
    name: &str,
    options: RegistryOptions,
  ) -> Result<VersionData, PackageError> {
    let target = options.target.unwrap_or(VersionTarget::Latest);

    let registry_url = options
      .registry_url
      .as_deref()
      .unwrap_or(&self.registry_url);

    let full_url = format!("{}/{}/{}", registry_url, name, target.to_str());
    let package_url = Url::parse(&full_url)?;

    let mut headers = HeaderMap::new();

    if options.full_metadata {
      headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    } else {
      headers.insert(
        ACCEPT,
        HeaderValue::from_static(
          "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
        ),
      );
    }

    let response = self
      .client
      .get(package_url)
      .headers(headers)
      .send()
      .await?
      .json::<VersionData>()
      .await
      .map_err(|_| PackageError::PackageNotFound(name.to_string()))?;

    Ok(response)
  }
}
