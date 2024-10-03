mod args;
mod fs_utils;
mod package_info;
mod package_json;
mod registry;
mod updater;

use anyhow::Result;
use clap::Parser;

use args::Args;
use package_json::PackageJsonManager;
use updater::UpdateChecker;

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();

  let mut pkg_manager = PackageJsonManager::new();
  pkg_manager.locate_closest()?;
  pkg_manager.read()?;

  let update_checker = UpdateChecker::new(args, pkg_manager);
  update_checker.run().await?;

  Ok(())
}
