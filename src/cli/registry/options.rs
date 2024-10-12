use super::version_target::VersionTarget;

#[derive(Debug)]
pub struct RegistryClientOptions {
  pub max_sockets: usize,
  pub timeout: u64,
  pub strict_ssl: bool,
}

impl Default for RegistryClientOptions {
  fn default() -> Self {
    RegistryClientOptions {
      max_sockets: 12,
      timeout: 5 * 60 * 1000,
      strict_ssl: true,
    }
  }
}

#[derive(Debug)]
pub struct RegistryOptions {
  pub target: Option<VersionTarget>,
  pub full_metadata: bool,
  pub registry_url: Option<String>,
}

impl Default for RegistryOptions {
  fn default() -> Self {
    RegistryOptions {
      target: Some(VersionTarget::Latest),
      full_metadata: true,
      registry_url: None,
    }
  }
}
