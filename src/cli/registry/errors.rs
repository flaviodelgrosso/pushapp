use thiserror::Error;

#[derive(Error, Debug)]
pub enum PackageError {
  #[error("Package `{0}` could not be found.")]
  PackageNotFound(String),
  #[error("HTTP request error: {0}")]
  RequestError(#[from] reqwest::Error),
  #[error("URL parse error: {0}")]
  ParseError(#[from] url::ParseError),
}
