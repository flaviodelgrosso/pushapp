use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(version, about, long_about = None)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
  /// Check only "devDependencies"
  #[clap(short, long)]
  pub development: bool,
  /// Check only "dependencies"
  #[clap(short, long)]
  pub production: bool,
  /// Check only "optionalDependencies"
  #[clap(short, long)]
  pub optional: bool,
  /// Check global packages
  #[clap(short, long)]
  pub global: bool,
}
