mod cli;
mod utils;

use anyhow::Result;
use clap::Parser;

use cli::args::Args;
use cli::package_json::PackageJsonManager;
use cli::updater::UpdateChecker;

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();

  let mut pkg_manager = PackageJsonManager::new();
  if !args.global {
    pkg_manager.locate_closest()?;
    pkg_manager.read()?;
  }

  let update_checker = UpdateChecker::new(args, pkg_manager);
  update_checker.run().await?;

  Ok(())
}
