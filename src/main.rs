mod args;
mod fs_utils;
mod package_info;
mod package_json;
mod package_lock;
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

  // Combine dependencies from package.json and devDependencies
  let deps = match pkg_manager.collect_deps(&args) {
    Ok(deps) => deps,
    Err(e) => {
      eprintln!("{e}");
      return Ok(());
    }
  };

  println!(
    "📦 {}",
    format!("Collecting {} dependencies.", deps.len())
      .bright_green()
      .bold(),
  );

  let message = if args.dev {
    "Checking updates... (dev dependencies only)"
  } else {
    "Checking updates..."
  };

  println!("🔍 {}", message.bright_yellow().bold());

  check_updates(pkg_manager, deps).await?;

  Ok(())
}
