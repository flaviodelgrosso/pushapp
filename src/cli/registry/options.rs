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
