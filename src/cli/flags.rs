use clap::Parser;

use super::versions::VersionTarget;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Flags {
  /// Check only "devDependencies".
  #[clap(short('D'), long)]
  pub development: bool,
  /// Check only "dependencies" and "optionalDependencies".
  #[clap(short('P'), long)]
  pub production: bool,
  /// Check global packages instead of in the current project.
  #[clap(short, long)]
  pub global: bool,
  /// Determines the version to upgrade to.
  #[clap(short, long, verbatim_doc_comment, default_value = "latest")]
  pub target: VersionTarget,
}
