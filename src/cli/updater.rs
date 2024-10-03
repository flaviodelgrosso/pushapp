use anyhow::Result;
use colored::Colorize;
use futures::stream::{FuturesUnordered, StreamExt};
use inquire::{formatter::MultiOptionFormatter, MultiSelect};
use semver::{Version, VersionReq};
use tokio::task;

use crate::cli::{
  args::Args,
  package_info::{normalize_version, PackageInfo},
  package_json::PackageJsonManager,
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

    let tasks = self.fetch_update_tasks();
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

  async fn process_update_results(
    &self,
    mut tasks: FuturesUnordered<task::JoinHandle<Option<PackageInfo>>>,
  ) -> Vec<PackageInfo> {
    let mut updatable_packages = vec![];

    while let Some(result) = tasks.next().await {
      if let Ok(Some(update_info)) = result {
        updatable_packages.push(update_info);
      } else if let Err(e) = result {
        eprintln!(
          "{}",
          format!("Task failed to execute to completion while checking updates: {e}").bright_red()
        );
      }
    }

    updatable_packages
  }

  async fn handle_updatable_packages(
    &self,
    mut updatable_packages: Vec<PackageInfo>,
  ) -> Result<()> {
    if updatable_packages.is_empty() {
      println!(
        "{}",
        "Good news! All packages are up-to-date.".bright_green()
      );
      return Ok(());
    }

    updatable_packages.sort_by(|a, b| a.pkg_name.cmp(&b.pkg_name));

    match display_update_prompt(updatable_packages) {
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

  fn fetch_update_tasks(&self) -> FuturesUnordered<task::JoinHandle<Option<PackageInfo>>> {
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
}

fn display_update_prompt(updatable_packages: Vec<PackageInfo>) -> Option<Vec<PackageInfo>> {
  let formatter: MultiOptionFormatter<'_, PackageInfo> =
    &|a| format!("{} package(s) selected", a.len());

  let prompt_message = format!(
    "Choose packages to update ({} total):",
    updatable_packages.len()
  );

  let prompt = MultiSelect::new(&prompt_message, updatable_packages)
    .with_formatter(&formatter)
    .prompt();

  match prompt {
    Ok(selected) => Some(selected),
    Err(_) => None,
  }
}

async fn get_package_info(
  client: &RegistryClient,
  name: &str,
  current_version: &str,
) -> Result<Option<PackageInfo>> {
  let latest_version = client.get_update_version(name, current_version).await?;

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
