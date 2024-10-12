use super::registry::version_target::VersionTarget;
use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
  /// Check only "devDependencies"
  #[clap(short('D'), long)]
  pub development: bool,
  /// Check only "dependencies" and "optionalDependencies"
  #[clap(short('P'), long)]
  pub production: bool,
  /// Check global packages instead of in the current project.
  #[clap(short, long)]
  pub global: bool,
  /// Set the registry URL
  #[clap(long, default_value = "https://registry.npmjs.org")]
  pub registry_url: Option<String>,
  /// Determines the version to upgrade to: latest, minor, patch, newest
  #[clap(short, long, default_value = "latest")]
  pub target: Option<VersionTarget>,
}
