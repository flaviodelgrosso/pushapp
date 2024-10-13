use anyhow::Result;
use nodejs_semver::{Range, Version};

use crate::cli::flags::Flags;

use super::VersionTarget;

pub fn normalize_version(version: &str) -> &str {
  version.trim_start_matches(|c: char| !c.is_numeric())
}

pub fn is_package_updatable(
  current_version: &str,
  latest_version: &str,
  flags: &Flags,
) -> Result<bool> {
  let curr_ver_range = Range::parse(current_version)?;
  let latest_ver = Version::parse(latest_version)?;
  let current_ver = Version::parse(normalize_version(current_version))?;

  // Check if an update can be made based on the flag
  let can_update = match flags.target {
    Some(VersionTarget::Latest) => {
      // Check if latest version is within the allowed range and newer
      if curr_ver_range.satisfies(&latest_ver) {
        latest_ver > current_ver
      } else {
        !current_ver.is_prerelease() // Do not allow prereleases if range is invalid
      }
    }
    Some(VersionTarget::Major) => latest_ver.major > current_ver.major,
    Some(VersionTarget::Minor) => {
      latest_ver.major == current_ver.major && latest_ver.minor > current_ver.minor
    }
    Some(VersionTarget::Patch) => {
      latest_ver.major == current_ver.major
        && latest_ver.minor == current_ver.minor
        && latest_ver.patch > current_ver.patch
    }
    Some(VersionTarget::Pre) => latest_ver.is_prerelease() && latest_ver > current_ver,
    None => false,
  };

  Ok(can_update)
}
