use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistryError {
  #[error("Package {0} could not be found: {1}")]
  PackageNotFound(String, reqwest::Error),
  #[error("HTTP request error: {0}")]
  RequestError(#[from] reqwest::Error),
  #[error("URL parse error: {0}")]
  ParseError(#[from] url::ParseError),
}
