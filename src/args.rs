use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
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
}
