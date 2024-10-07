pub mod errors;
pub mod options;
pub mod version_target;

use anyhow::Result;
use errors::PackageError;
use options::Options;
use reqwest::{
  header::{HeaderMap, HeaderValue, ACCEPT},
  Client,
};
use serde::{Deserialize, Serialize};
use url::Url;
use version_target::VersionTarget;

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct RegistryClient {
  pub client: Client,
  pub registry_url: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct VersionData {
  version: String,
  deprecated: Option<bool>,
  time: Option<String>,
}

impl RegistryClient {
  pub fn new() -> Self {
    RegistryClient {
      client: Client::new(),
      registry_url: "https://registry.npmjs.org".to_string(),
    }
  }

  async fn fetch_registry(
    &self,
    name: &str,
    options: Options,
  ) -> Result<VersionData, PackageError> {
    let target = options.target.unwrap_or(VersionTarget::Latest);
    let registry_url = options
      .registry_url
      .unwrap_or_else(|| format!("{}/{}/{}", self.registry_url, name, target.to_str()));

    let package_url = Url::parse(&registry_url)?;

    let mut headers = HeaderMap::new();
    headers.insert(
      ACCEPT,
      HeaderValue::from_static(
        "application/vnd.npm.install-v1+json; q=1.0, application/json; q=0.8, */*",
      ),
    );

    if options.full_metadata {
      headers.remove(ACCEPT);
      headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
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

  pub async fn get_package_version(
    &self,
    name: &str,
    target: Option<VersionTarget>,
  ) -> Result<String, PackageError> {
    let options = Options {
      full_metadata: true,
      target,
      ..Default::default()
    };

    let version_data = self.fetch_registry(name, options).await?;

    Ok(version_data.version)
  }
}
