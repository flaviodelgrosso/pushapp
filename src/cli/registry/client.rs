use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use reqwest::{Client, ClientBuilder};
use url::Url;

use super::{RegistryClientOptions, RegistryError};

use crate::cli::{
  flags::Flags,
  package_info::PackageInfo,
  versions::{is_package_updatable, DistTags, VersionTarget},
};

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

    if is_package_updatable(current_version, &latest_version, flags)? {
      Ok(Some(PackageInfo {
        pkg_name: name.to_string(),
        current_version: current_version.to_string(),
        latest_version: latest_version.to_string(),
      }))
    } else {
      Ok(None)
    }
  }

  async fn fetch_package_version(
    &self,
    name: &str,
    flags: &Flags,
  ) -> Result<String, RegistryError> {
    let dist_tags = self.fetch_registry(name).await?;
    let version_match = Self::match_dist_tag_with_target(dist_tags, &flags.target);

    Ok(version_match)
  }

  async fn fetch_registry(&self, name: &str) -> Result<DistTags, RegistryError> {
    let full_url = format!("{}/-/package/{}/dist-tags", self.registry_url, name);
    let package_url = Url::parse(&full_url)?;

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let response = self
      .client
      .get(package_url)
      .headers(headers)
      .send()
      .await?
      .json::<DistTags>()
      .await
      .map_err(|e| RegistryError::PackageNotFound(name.to_string(), e))?;

    Ok(response)
  }

  fn match_dist_tag_with_target(dist_tags: DistTags, target: &Option<VersionTarget>) -> String {
    match target {
      Some(VersionTarget::Pre) => dist_tags
        .next
        .or(dist_tags.canary)
        .or(dist_tags.rc)
        .or(dist_tags.beta)
        .or(dist_tags.alpha)
        .unwrap_or(dist_tags.latest),
      _ => dist_tags.latest,
    }
  }
}
