use anyhow::Result;
use nodejs_semver::{Range, Version};

use crate::cli::flags::Flags;

use super::{normalize_version, DistTags, VersionTarget};

pub fn match_dist_tag_with_target(dist_tags: DistTags, target: &VersionTarget) -> String {
  match target {
    VersionTarget::Pre => dist_tags
      .next
      .or(dist_tags.canary)
      .or(dist_tags.rc)
      .or(dist_tags.beta)
      .or(dist_tags.alpha)
      .unwrap_or(dist_tags.latest),
    _ => dist_tags.latest,
  }
}

pub fn is_version_satisfying(
  current_version: &str,
  latest_version: &str,
  flags: &Flags,
) -> Result<bool> {
  let latest_ver = Version::parse(latest_version)?;
  let current_ver = Version::parse(normalize_version(current_version))?;

  // Check if an update can be made based on the flag
  let matching_version = match flags.target {
    VersionTarget::Latest => {
      Range::parse(current_version)?.satisfies(&latest_ver) && latest_ver > current_ver
    }
    VersionTarget::Major => latest_ver.major > current_ver.major,
    VersionTarget::Minor => {
      latest_ver.major == current_ver.major && latest_ver.minor > current_ver.minor
    }
    VersionTarget::Patch => {
      latest_ver.major == current_ver.major
        && latest_ver.minor == current_ver.minor
        && latest_ver.patch > current_ver.patch
    }
    VersionTarget::Pre => latest_ver.is_prerelease() && latest_ver > current_ver,
  };

  Ok(matching_version)
}
