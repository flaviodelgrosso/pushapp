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
  /// Check global packages
  #[clap(short, long)]
  pub global: bool,
}
