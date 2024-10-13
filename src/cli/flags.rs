use clap::Parser;

use super::versions::VersionTarget;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Flags {
  /// Check only "devDependencies"
  #[clap(short('D'), long)]
  pub development: bool,
  /// Check only "dependencies" and "optionalDependencies"
  #[clap(short('P'), long)]
  pub production: bool,
  /// Check global packages instead of in the current project.
  #[clap(short, long)]
  pub global: bool,
  /// Determines the version to upgrade to.
  /// 1) `latest` means the latest version available in the registry, excluding pre-releases.
  /// 2) `semver` means upgrade to the highest version within the semver range specified in your package.json.
  /// 3) `major` means upgrade to the highest major version.
  /// 4) `minor` means upgrade to the highest minor version without bumping the major version.
  /// 5) `patch` means upgrade to the highest patch version without bumping the major or minor version.
  #[clap(short, long, verbatim_doc_comment, default_value = "latest")]
  pub target: VersionTarget,
}
