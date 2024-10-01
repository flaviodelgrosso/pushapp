use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct RegistryClient {
  pub client: Client,
  pub registry_url: String,
}

#[derive(Debug, Deserialize)]
struct DistTags {
  latest: String,
  next: Option<String>,
  canary: Option<String>,
  alpha: Option<String>,
  beta: Option<String>,
  experimental: Option<String>,
  rc: Option<String>,
}

impl RegistryClient {
  pub fn new() -> Self {
    RegistryClient {
      client: Client::new(),
      registry_url: "https://registry.npmjs.org".to_string(),
    }
  }

  async fn get_dist_tags(&self, name: &str) -> Result<DistTags> {
    let url = format!("{}/-/package/{}/dist-tags", self.registry_url, name);
    let response = self
      .client
      .get(&url)
      .send()
      .await?
      .json::<DistTags>()
      .await?;

    Ok(response)
  }

  pub async fn get_update_version(&self, name: &str, current_version: &str) -> Result<String> {
    let dist_tags = self.get_dist_tags(name).await?;

    // if the current version includes a prerelease tag, check if the
    // prerelease tag is available in the dist-tags. if it is, return the prerelease version. otherwise return the latest version.
    if current_version.contains('-') {
      let parts: Vec<&str> = current_version.split('-').collect();
      let prerelease_tag = parts[1].split('.').next().unwrap_or("");
      let prerelease_version = match prerelease_tag {
        "next" => dist_tags.next,
        "canary" => dist_tags.canary,
        "alpha" => dist_tags.alpha,
        "beta" => dist_tags.beta,
        "experimental" => dist_tags.experimental,
        "rc" => dist_tags.rc,
        _ => None,
      };

      if let Some(version) = prerelease_version {
        return Ok(version);
      }
    }

    Ok(dist_tags.latest)
  }
}
