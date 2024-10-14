use anyhow::Result;
use nodejs_semver::{Range, Version, VersionDiff};

use crate::cli::flags::Flags;

use super::{normalize_version, DistTags, VersionTarget};

pub fn match_dist_tag_with_target(dist_tags: DistTags, target: &VersionTarget) -> Option<String> {
  match target {
    VersionTarget::Pre => dist_tags.highest_prerelease_version(),
    VersionTarget::Next => dist_tags.next,
    VersionTarget::Canary => dist_tags.canary,
    VersionTarget::Rc => dist_tags.rc,
    VersionTarget::Beta => dist_tags.beta,
    VersionTarget::Alpha => dist_tags.alpha,
    _ => Some(dist_tags.latest),
  }
}

pub fn is_version_satisfying(
  current_version: &str,
  latest_version: &str,
  flags: &Flags,
) -> Result<bool> {
  let current = Version::parse(normalize_version(current_version))?;
  let latest = Version::parse(latest_version)?;

  let diff = current.diff(&latest);
  if diff.is_none() || latest <= current {
    return Ok(false);
  }

  let matching_version = match flags.target {
    VersionTarget::Latest => !current.is_prerelease(),
    VersionTarget::Semver => Range::parse(current_version)?.satisfies(&latest),
    VersionTarget::Major => diff == Some(VersionDiff::Major),
    VersionTarget::Minor => diff == Some(VersionDiff::Minor),
    VersionTarget::Patch => diff == Some(VersionDiff::Patch),
    _ => latest.is_prerelease(),
  };

  Ok(matching_version)
}
