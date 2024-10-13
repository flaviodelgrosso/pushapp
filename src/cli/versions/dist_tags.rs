use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DistTags {
  pub latest: String,
  pub next: Option<String>,
  pub canary: Option<String>,
  pub rc: Option<String>,
  pub beta: Option<String>,
  pub alpha: Option<String>,
}
