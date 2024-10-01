mod fs_utils;
mod package_info;
mod package_json;
mod package_lock;
mod registry;
mod updater;

use anyhow::Result;
use colored::Colorize;

use package_json::PackageJsonManager;
use updater::check_updates;

#[tokio::main]
async fn main() -> Result<()> {
  let mut pkg_manager = PackageJsonManager::new();
  pkg_manager.locate_closest()?;
  pkg_manager.read()?;

  // Combine dependencies from package.json and devDependencies
  let all_deps = match pkg_manager.combine_deps() {
    Ok(deps) => deps,
    Err(e) => {
      eprintln!("{e}");
      return Ok(());
    }
  };

  println!(
    "📦 {}",
    format!("Found {} total dependencies", all_deps.len())
      .bright_green()
      .bold(),
  );

  println!(
    "🔍 {}",
    "Checking for packages updates...".bright_yellow().bold(),
  );

  check_updates(pkg_manager, all_deps).await?;

  Ok(())
}
