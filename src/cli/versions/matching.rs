use anyhow::Result;
use nodejs_semver::{Range, Version, VersionDiff};

use crate::cli::flags::Flags;

use super::{normalize_version, DistTags, VersionTarget};

pub fn match_dist_tag_with_target(dist_tags: DistTags, target: &VersionTarget) -> String {
  match target {
    // TODO: Keep the highest pre-release version
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
  let current = Version::parse(normalize_version(current_version))?;
  let latest = Version::parse(latest_version)?;

  let diff = current.diff(&latest);

  let matching_version = match flags.target {
    VersionTarget::Latest => diff.is_some(),
    VersionTarget::Semver => Range::parse(current_version)?.satisfies(&latest) && diff.is_some(),
    VersionTarget::Major => diff == Some(VersionDiff::Major),
    VersionTarget::Minor => diff == Some(VersionDiff::Minor),
    VersionTarget::Patch => diff == Some(VersionDiff::Patch),
    VersionTarget::Pre => latest.is_prerelease() && latest > current,
  };

  Ok(matching_version)
}
