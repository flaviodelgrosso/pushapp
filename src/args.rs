use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// Only check for updates in devDependencies
  #[clap(short('D'), long)]
  pub dev: bool,
}
