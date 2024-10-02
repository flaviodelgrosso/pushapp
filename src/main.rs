mod args;
mod fs_utils;
mod package_info;
mod package_json;
mod registry;
mod updater;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use args::Args;
use package_json::PackageJsonManager;
use updater::check_updates;

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();

  let mut pkg_manager = PackageJsonManager::new();
  pkg_manager.locate_closest()?;
  pkg_manager.read()?;

  println!("🔍 {}", "Checking updates...".bright_yellow());

  check_updates(&args, &pkg_manager).await?;

  Ok(())
}
