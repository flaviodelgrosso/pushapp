use std::sync::Arc;

use anyhow::Result;
use colored::Colorize;
use futures::{stream::FuturesUnordered, StreamExt};
use semver::{Version, VersionReq};
use tokio::task::{self, JoinHandle};

use super::{
  flags::Flags,
  package_info::{normalize_version, PackageInfo},
  package_json::{PackageDependencies, PackageJsonManager},
  prompt::display_update,
  registry::RegistryClient,
};

#[derive(Debug)]
pub struct UpdateChecker {
  pkg_manager: PackageJsonManager,
  client: Arc<RegistryClient>,
  flags: Arc<Flags>,
}

impl UpdateChecker {
  pub fn new(pkg_manager: PackageJsonManager, flags: Arc<Flags>) -> Self {
    Self {
      pkg_manager,
      flags,
      client: Arc::new(RegistryClient::default()),
    }
  }

  pub async fn run(&self) -> Result<()> {
    println!("🔍 {}", "Checking updates...".bright_yellow());

    let deps = if self.flags.global {
      self::PackageJsonManager::get_global_deps()?
    } else {
      self.pkg_manager.get_local_deps()
    };

    let tasks = self.fetch_updates(deps);
    if tasks.is_empty() {
      println!("{}", "📦 No dependencies found.".bright_red());
      return Ok(());
    }

    println!(
      "{}",
      format!("📦 Found {} dependencies.", tasks.len()).bright_green()
    );

    let updatable_packages = self.process_update_stream(tasks).await;
    self.handle_updatable_packages(updatable_packages)
  }

  fn fetch_updates(
    &self,
    deps: PackageDependencies,
  ) -> FuturesUnordered<JoinHandle<Option<PackageInfo>>> {
    deps
      .into_iter()
      .map(|(name, version)| {
        let client = self.client.clone();
        let flags = self.flags.clone();
        task::spawn(async move {
          match client.get_package_info(&name, &version, &flags).await {
            Ok(Some(info)) => Some(info),
            Ok(None) => None,
            Err(e) => {
              eprintln!("{}", format!("❌ {e}").bright_red());
              None
            }
          }
        })
      })
      .collect()
  }

  async fn process_update_stream(
    &self,
    mut tasks: FuturesUnordered<JoinHandle<Option<PackageInfo>>>,
  ) -> Vec<PackageInfo> {
    let mut pkg_infos = Vec::new();

    // Process each task as it completes
    while let Some(task) = tasks.next().await {
      match task {
        Ok(Some(pkg_info)) => pkg_infos.push(pkg_info),
        Ok(None) => {} // Skip None results
        Err(e) => {
          eprintln!("❌ Task failed to execute: {e}");
        }
      }
    }

    pkg_infos
  }

  fn handle_updatable_packages(&self, mut updatable_packages: Vec<PackageInfo>) -> Result<()> {
    if updatable_packages.is_empty() {
      println!("{}", "There are no updates available.".bright_blue());
      return Ok(());
    }

    updatable_packages.sort_by(|a, b| a.pkg_name.cmp(&b.pkg_name));

    match display_update(updatable_packages) {
      Some(selected) => {
        self.pkg_manager.install_deps(&selected)?;
      }
      None => {
        println!("{}", "\nNo packages were updated.".bright_yellow());
      }
    }

    Ok(())
  }
}

/// Determines whether an update is needed based on the version requirements and the latest version available.
///
/// If the latest version satisfies the version requirement, an update is needed only if the latest version is greater than the current version.
/// Otherwise, an update is always needed.
///
/// # Parameters
/// - `version_req`: The version requirement that the latest version must satisfy.
/// - `latest_ver`: The latest version available.
/// - `current_ver`: The current version in use.
///
/// # Returns
/// - `true` if an update is needed.
/// - `false` if no update is needed.
pub fn can_update(current_version: &str, latest_version: &str) -> Result<bool> {
  // Remove any caret or tilde from current version before parsing
  let cleaned_current_version = normalize_version(current_version);
  let version_req = VersionReq::parse(cleaned_current_version)?;
  let current_ver = Version::parse(cleaned_current_version)?;
  let latest_ver = Version::parse(latest_version)?;

  let needs_update = if version_req.matches(&latest_ver) {
    latest_ver > current_ver // True if an update is needed
  } else {
    true // Update needed if latest version doesn't satisfy the current version constraint
  };

  Ok(needs_update)
}
