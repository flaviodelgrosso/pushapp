use anyhow::{format_err, Result};
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use tokio::process::Command;

use super::{
  args::Args,
  fs_utils::{find_closest_file, read_json},
  package_info::PackageInfo,
};

pub type PackageDependencies = HashMap<String, String>;

static PACKAGE_JSON_FILENAME: &str = "package.json";

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
  pub dependencies: Option<PackageDependencies>,
  pub dev_dependencies: Option<PackageDependencies>,
  pub optional_dependencies: Option<PackageDependencies>,
  pub package_manager: Option<String>,
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

  pub fn all_deps_iter(&self, args: &Args) -> impl Iterator<Item = (&String, &String)> {
    // Build a vector of the selected dependencies based on CLI arguments
    let mut selected_deps: Vec<&Option<PackageDependencies>> = Vec::new();

    if args.production || (!args.development && !args.optional) {
      selected_deps.push(&self.json.dependencies);
    }

    if args.development || (!args.production && !args.optional) {
      selected_deps.push(&self.json.dev_dependencies);
    }

    if args.optional || (!args.production && !args.development) {
      selected_deps.push(&self.json.optional_dependencies);
    }

    selected_deps
      .into_iter()
      .flat_map(|deps_option| deps_option.iter().flat_map(|deps| deps.iter()))
  }

  /// Detect the package manager used in the project and return it with the install command.
  fn detect_package_manager(&self) -> (String, String) {
    let package_manager = self.json.package_manager.as_deref().unwrap_or("npm");

    // Split at '@' and get the package manager name
    let package_manager_name = package_manager.split('@').next().unwrap_or("npm");

    // Determine the command based on the package manager
    let command = match package_manager_name {
      "npm" => "install",
      _ => "add",
    };

    (package_manager_name.to_string(), command.to_string())
  }

  pub async fn install_deps(&self, updates: Vec<PackageInfo>) -> Result<()> {
    let (package_manager, command) = self.detect_package_manager();

    let install_args = updates
      .iter()
      .map(|package| format!("{}@{}", package.pkg_name, package.latest_version))
      .collect::<Vec<String>>();

    #[cfg(not(debug_assertions))]
    let status = Command::new(package_manager)
      .arg(command)
      .args(install_args)
      .status()
      .await?;

    #[cfg(debug_assertions)]
    let status = Command::new("echo")
      .arg(format!(
        "Would have run: {} {} {}",
        package_manager,
        command,
        install_args.join(" ")
      ))
      .status()
      .await?;

    if status.success() {
      println!("{}", "Packages successfully updated!".bright_green());
    } else {
      anyhow::bail!("Failed to update packages");
    }

    Ok(())
  }
}
