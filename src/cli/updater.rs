use anyhow::Result;
use colored::Colorize;
use futures::future::join_all;
use semver::{Version, VersionReq};
use tokio::task::{self, JoinHandle};

use super::{
  args::Args,
  package_info::{normalize_version, PackageInfo},
  package_json::PackageJsonManager,
  prompt::display_update,
  registry::RegistryClient,
};

#[derive(Debug)]
pub struct UpdateChecker {
  args: Args,
  pkg_manager: PackageJsonManager,
  client: RegistryClient,
}

impl UpdateChecker {
  pub fn new(args: Args, pkg_manager: PackageJsonManager) -> Self {
    Self {
      args,
      pkg_manager,
      client: RegistryClient::new(),
    }
  }

  pub async fn run(&self) -> Result<()> {
    println!("🔍 {}", "Checking updates...".bright_yellow());

    let tasks = self.fetch_updates();
    if tasks.is_empty() {
      println!("{}", "📦 No dependencies found.".bright_red());
      return Ok(());
    }

    println!(
      "{}",
      format!("📦 Found {} dependencies.", tasks.len()).bright_green()
    );

    let updatable_packages = self.process_update_results(tasks).await;
    self.handle_updatable_packages(updatable_packages).await
  }

  fn fetch_updates(&self) -> Vec<JoinHandle<Option<PackageInfo>>> {
    self
      .pkg_manager
      .all_deps_iter(&self.args)
      .map(|(name, version)| {
        let client = self.client.clone();
        let name = name.clone();
        let version = version.clone();
        task::spawn(async move {
          match get_package_info(&client, &name, &version).await {
            Ok(Some(info)) => Some(info),
            Ok(None) => None,
            Err(e) => {
              eprintln!(
                "{}",
                format!("❌ Error checking updates for package {name}: {e}").bright_red()
              );
              None
            }
          }
        })
      })
      .collect()
  }

  async fn process_update_results(
    &self,
    tasks: Vec<JoinHandle<Option<PackageInfo>>>,
  ) -> Vec<PackageInfo> {
    // Use join_all to await all the task concurrently
    join_all(tasks)
      .await
      .into_iter()
      .filter_map(|res| match res {
        Ok(Some(result)) => Some(result),
        Ok(None) => None,
        Err(e) => {
          eprintln!("❌ Task failed to execute: {e}");
          None
        }
      })
      .collect()
  }

  async fn handle_updatable_packages(
    &self,
    mut updatable_packages: Vec<PackageInfo>,
  ) -> Result<()> {
    if updatable_packages.is_empty() {
      println!("{}", "There are no updates available.".bright_blue());
      return Ok(());
    }

    updatable_packages.sort_by(|a, b| a.pkg_name.cmp(&b.pkg_name));

    match display_update(updatable_packages) {
      Some(selected) => {
        if selected.is_empty() {
          println!(
            "{}",
            "No packages were selected for update.".bright_yellow()
          );
        } else {
          self.pkg_manager.install_deps(selected).await?;
        }
      }
      None => {
        println!("{}", "\nNo packages were updated.".bright_yellow());
      }
    }

    Ok(())
  }
}

async fn get_package_info(
  client: &RegistryClient,
  name: &str,
  current_version: &str,
) -> Result<Option<PackageInfo>> {
  let latest_version = client.get_latest_version(name, current_version).await?;

  if can_update(current_version, &latest_version)? {
    Ok(Some(PackageInfo {
      pkg_name: name.to_string(),
      current_version: current_version.to_string(),
      latest_version: latest_version.to_string(),
    }))
  } else {
    Ok(None)
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
fn can_update(current_version: &str, latest_version: &str) -> Result<bool> {
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
