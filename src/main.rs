#![allow(clippy::module_name_repetitions)]

mod cli;
mod utils;

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;

use cli::flags::Flags;
use cli::package_json::PackageJsonManager;
use cli::updater::UpdateChecker;

#[tokio::main]
async fn main() -> Result<()> {
  let flags = Arc::new(Flags::parse());

  let mut pkg_manager = PackageJsonManager::new(flags.clone());
  if !flags.global {
    pkg_manager.locate_closest()?;
    pkg_manager.read()?;
  }

  let update_checker = UpdateChecker::new(pkg_manager, flags.clone());
  update_checker.run().await?;

  Ok(())
}
