use anyhow::{format_err, Result};
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::{
  args::Args,
  fs_utils::{find_closest_file, read_json},
  package_info::PackageInfo,
};

pub type PackageDependencies = HashMap<String, String>;

pub static PACKAGE_JSON_FILENAME: &str = "package.json";

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
  pub dependencies: Option<PackageDependencies>,
  pub dev_dependencies: Option<PackageDependencies>,
  pub optional_dependencies: Option<PackageDependencies>,
  pub package_manager: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GlobalPackage {
  pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct GlobalList {
  pub dependencies: HashMap<String, GlobalPackage>,
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

  pub fn get_local_deps(&self, args: &Args) -> PackageDependencies {
    let mut combined_deps = PackageDependencies::new();

    // Apply logic based on the provided flags
    if args.production || (!args.development && !args.optional) {
      if let Some(dependencies) = &self.json.dependencies {
        combined_deps.extend(dependencies.clone());
      }
    }

    if args.development || (!args.production && !args.optional) {
      if let Some(dev_dependencies) = &self.json.dev_dependencies {
        combined_deps.extend(dev_dependencies.clone());
      }
    }

    if args.optional || (!args.production && !args.development) {
      if let Some(optional_dependencies) = &self.json.optional_dependencies {
        combined_deps.extend(optional_dependencies.clone());
      }
    }

    combined_deps
  }

  pub fn get_global_deps() -> Result<PackageDependencies> {
    // Run the `npm list -g --depth=0` command to get the global packages and their versions
    let output = Command::new("npm")
      .arg("ls")
      .arg("--json")
      .arg("-g")
      .arg("--depth=0")
      .output()?;

    let output_str = String::from_utf8(output.stdout)?;

    let global_list: GlobalList = serde_json::from_str(&output_str)?;

    let packages = global_list
      .dependencies
      .iter()
      .map(|(name, package)| (name.clone(), package.version.clone()))
      .collect();

    Ok(packages)
  }

  /// Detect the package manager used in the project and return it with the install command.
  fn detect_package_manager(&self, args: &Args) -> Result<String> {
    if args.global {
      return Ok("npm".into());
    }

    if let Some(manager) = self.get_package_manager_from_json() {
      return Ok(manager);
    }

    if let Some(manager) = Self::detect_lock_file()? {
      return Ok(manager);
    }

    Ok("npm".into())
  }

  fn get_package_manager_from_json(&self) -> Option<String> {
    self
      .json
      .package_manager
      .as_ref()
      .and_then(|pm| pm.split('@').next().map(std::convert::Into::into))
  }

  fn detect_lock_file() -> Result<Option<String>> {
    let cwd = env::current_dir()?;
    let lock_files = [
      ("yarn.lock", "yarn"),
      ("package-lock.json", "npm"),
      ("pnpm-lock.yaml", "pnpm"),
      ("bun.lockb", "bun"),
    ];

    for (lock_file, manager) in lock_files {
      if cwd.join(lock_file).exists() {
        return Ok(Some(manager.into()));
      }
    }

    Ok(None)
  }

  pub fn install_deps(&self, updates: &[PackageInfo], args: &Args) -> Result<()> {
    let package_manager = self.detect_package_manager(args)?;
    let install_args = Self::construct_install_args(updates);

    let command = Self::determine_install_command(&package_manager);

    Self::execute_install_command(&package_manager, command, install_args, args.global)?;

    Ok(())
  }

  fn construct_install_args(updates: &[PackageInfo]) -> Vec<String> {
    updates
      .iter()
      .map(|package| format!("{}@{}", package.pkg_name, package.latest_version))
      .collect()
  }

  fn determine_install_command(package_manager: &str) -> &str {
    match package_manager {
      "npm" => "install",
      _ => "add",
    }
  }

  fn execute_install_command(
    package_manager: &str,
    command: &str,
    install_args: Vec<String>,
    global: bool,
  ) -> Result<()> {
    let mut cmd = Command::new(package_manager);
    cmd.arg(command).args(install_args);

    if global {
      cmd.arg("-g");
    }

    let status = cmd.status()?;
    if status.success() {
      println!("{}", "Packages successfully updated!".bright_green());
    } else {
      anyhow::bail!(
        "Failed to update packages using {} command for manager: {}",
        command,
        package_manager
      );
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_spec_fields() {
    let package_json_raw = r#"
    {
      "packageManager": "pnpm@9.10.0",
      "devDependencies": {
        "typescript": "*"
      }
    }
  "#;

    let json = serde_json::from_str::<PackageJson>(package_json_raw).unwrap();
    assert_eq!(json.package_manager, Some("pnpm@9.10.0".to_owned()));
    assert_eq!(json.dependencies, None);
    assert_eq!(
      json.dev_dependencies,
      Some(HashMap::from([("typescript".to_owned(), "*".to_owned())]))
    );
  }

  #[test]
  fn test_detect_package_manager() {
    let package_json = PackageJson {
      package_manager: Some("pnpm@9.10.0".to_owned()),
      ..Default::default()
    };

    let manager = PackageJsonManager {
      json: package_json,
      ..Default::default()
    };

    let args = Args::default();

    assert_eq!(manager.detect_package_manager(&args).unwrap(), "pnpm");
  }
}
