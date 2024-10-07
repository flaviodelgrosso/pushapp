use super::version_target::VersionTarget;

pub struct Options {
  pub target: Option<VersionTarget>,
  pub full_metadata: bool,
  pub registry_url: Option<String>,
}

impl Default for Options {
  fn default() -> Self {
    Options {
      target: Some(VersionTarget::Latest),
      full_metadata: false,
      registry_url: None,
    }
  }
}
