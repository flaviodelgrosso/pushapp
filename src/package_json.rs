use anyhow::{anyhow, format_err, Result};
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use tokio::process::Command;

use crate::args::Args;
use crate::fs_utils::{find_closest_file, read_json};
use crate::package_info::PackageInfo;
use crate::package_lock::PackageLock;

pub type PackageDependencies = HashMap<String, String>;

static PACKAGE_JSON_FILENAME: &str = "package.json";

#[derive(Deserialize, Debug, Default)]
pub struct PackageJson {
  pub dependencies: Option<PackageDependencies>,
  #[serde(rename = "devDependencies")]
  pub dev_dependencies: Option<PackageDependencies>,
}

#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct PackageJsonManager {
  pub file_path: Option<PathBuf>,
  pub json: PackageJson,
}

impl PackageJsonManager {
  pub fn new() -> Self {
    PackageJsonManager::default()
  }

  /// Try to locate the closest `package.json` file from [current working directory][std::env::current_dir] to sys root.
  pub fn locate_closest(&mut self) -> Result<PathBuf> {
    env::current_dir().map(|cwd| self.locate_closest_from(cwd))?
  }

  /// Try to locate the closest `package.json` file from specific directory to sys root.
  fn locate_closest_from<P: AsRef<Path>>(&mut self, from: P) -> Result<PathBuf> {
    find_closest_file(PACKAGE_JSON_FILENAME, from).map(|file_path| {
      self.file_path = Some(file_path);
      self.file_path.as_ref().unwrap().to_owned()
    })
  }

  /// Call file reader to read `package.json` file.
  pub fn read(&mut self) -> Result<()> {
    match self.file_path.as_ref() {
      Some(file_path) => read_json(file_path).map(|json| {
        self.json = json;
      }),
      None => Err(format_err!(
        "Couldn't find an available {} file.",
        PACKAGE_JSON_FILENAME
      )),
    }
  }

  /// Combine both dependencies and devDependencies into one `HashMap` if `dev` flag is not set.
  pub fn collect_deps(&self, args: &Args) -> Result<PackageDependencies> {
    let mut deps = PackageDependencies::new();

    if !args.dev {
      if let Some(dependencies) = &self.json.dependencies {
        deps.extend(dependencies.clone());
      }
    }

    if let Some(dev_dependencies) = &self.json.dev_dependencies {
      deps.extend(dev_dependencies.clone());
    }

    if deps.is_empty() {
      return Err(anyhow!("{}", "No dependencies found.".bright_red().bold()));
    }

    Ok(deps)
  }

  fn detect_package_manager() -> (String, String) {
    // Array of tuples with lock file names and corresponding package managers
    let lock_files = [
      ("package-lock.json", PackageLock::Npm),
      ("yarn.lock", PackageLock::Yarn),
      ("pnpm-lock.yaml", PackageLock::Pnpm),
      ("bun.lockb", PackageLock::Bun),
    ];

    // Loop through lock files and return the first match
    for (file, manager) in &lock_files {
      if let Ok(true) = Path::new(file).try_exists() {
        let command = match manager {
          PackageLock::Npm => "install",
          _ => "add",
        };
        return (manager.to_string(), command.to_string());
      }
    }

    // Default to Npm if no lock files are found
    (PackageLock::Npm.to_string(), "install".to_string())
  }

  pub async fn install_deps(&self, updates: Vec<PackageInfo>) -> Result<()> {
    let (package_manager, command) = PackageJsonManager::detect_package_manager();

    let install_args = updates
      .iter()
      .map(|package| format!("{}@{}", package.pkg_name, package.latest_version))
      .collect::<Vec<String>>();

    let status = Command::new(package_manager)
      .arg(command)
      .args(install_args)
      .status()
      .await?;

    if status.success() {
      println!("{}", "Packages successfully updated!".bright_green().bold());
    } else {
      anyhow::bail!("Failed to update packages");
    }

    Ok(())
  }
}
