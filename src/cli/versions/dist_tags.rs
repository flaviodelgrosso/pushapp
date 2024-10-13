use nodejs_semver::Version;
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

impl DistTags {
  /// This function returns the highest semantic pre-release version found in the `DistTags` struct.
  pub fn highest_prerelease_version(&self) -> Option<String> {
    // Collect only the optional pre-release versions (excluding `latest`)
    let prerelease_versions = vec![
      self.next.as_ref(),
      self.canary.as_ref(),
      self.rc.as_ref(),
      self.beta.as_ref(),
      self.alpha.as_ref(),
    ];

    let mut parsed_versions: Vec<Version> = prerelease_versions
      .into_iter()
      .filter_map(|v| v.and_then(|s| Version::parse(s).ok())) // Parse and filter invalid ones
      .collect();

    // Sort the versions and get the highest pre-release one
    parsed_versions.sort();
    parsed_versions.pop().map(|v| v.to_string()) // Return the highest one as a string
  }
}
