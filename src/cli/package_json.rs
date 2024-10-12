use anyhow::{format_err, Result};
use colored::Colorize;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;

use super::{
  flags::Flags,
  package_info::PackageInfo,
  package_manager::{PackageManager, BUN_LOCK, NPM_LOCK, PNPM_LOCK, YARN_LOCK},
};
use crate::utils::{
  fs::{find_closest_file, read_json},
  hashmap::merge,
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
pub struct GlobalDependency {
  pub version: String,
}

#[derive(Deserialize, Debug)]
pub struct GlobalDependencies {
  pub dependencies: HashMap<String, GlobalDependency>,
}

#[derive(Debug, Default)]
pub struct PackageJsonManager {
  pub file_path: Option<PathBuf>,
  pub json: PackageJson,
  pub flags: Arc<Flags>,
}

impl PackageJsonManager {
  pub fn new(flags: Arc<Flags>) -> Self {
    Self {
      flags,
      ..Default::default()
    }
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

  pub fn get_local_deps(&self) -> PackageDependencies {
    let dependencies = if self.flags.production {
      // Return production dependencies
      vec![
        self.json.dependencies.as_ref(),
        self.json.optional_dependencies.as_ref(),
      ]
    } else if self.flags.development {
      // Return development dependencies
      vec![self.json.dev_dependencies.as_ref()]
    } else {
      // Default to all dependencies
      vec![
        self.json.dependencies.as_ref(),
        self.json.dev_dependencies.as_ref(),
        self.json.optional_dependencies.as_ref(),
      ]
    };

    merge(&dependencies)
  }

  pub fn get_global_deps() -> Result<PackageDependencies> {
    // Run the `npm list -g --depth=0` command
    let output = Command::new("npm")
      .args(["ls", "--json", "-g", "--depth=0"])
      .output()?;

    let global_deps: GlobalDependencies = serde_json::from_slice(&output.stdout)?;

    // Map the dependencies to a name -> version structure
    let packages = global_deps
      .dependencies
      .into_iter()
      .map(|(name, package)| (name, package.version))
      .collect();

    Ok(packages)
  }

  /// Detect the package manager based on the provided flags, package.json, and lock files.
  fn detect_package_manager(&self) -> PackageManager {
    if self.flags.global {
      return PackageManager::Npm;
    }

    if let Some(manager) = self.get_package_manager_from_json() {
      return manager;
    }

    if let Some(manager) = self.detect_lock_file() {
      return manager;
    }

    PackageManager::Npm
  }

  fn get_package_manager_from_json(&self) -> Option<PackageManager> {
    let package_manager = self.json.package_manager.as_ref()?.split('@').next()?;
    Some(PackageManager::from(package_manager))
  }

  fn detect_lock_file(&self) -> Option<PackageManager> {
    let lock_files = [NPM_LOCK, YARN_LOCK, PNPM_LOCK, BUN_LOCK];

    // Ensure file_path exists before proceeding
    let file_path = self.file_path.as_ref()?;

    // Iterate over lock files and check existence
    lock_files.iter().find_map(|&lock_file| {
      let candidate_path = file_path.with_file_name(lock_file);
      if candidate_path.exists() {
        Some(PackageManager::from_lock_file(lock_file))
      } else {
        None
      }
    })
  }

  pub fn install_deps(&self, updates: &[PackageInfo]) -> Result<()> {
    let package_manager = self.detect_package_manager();
    let install_args = Self::construct_install_args(updates);
    let command = PackageManager::determine_install_command(&package_manager);

    self.execute_install_command(&package_manager, command, install_args)?;

    Ok(())
  }

  fn construct_install_args(updates: &[PackageInfo]) -> Vec<String> {
    updates
      .iter()
      .map(|package| format!("{}@{}", package.pkg_name, package.latest_version))
      .collect()
  }

  fn execute_install_command(
    &self,
    package_manager: &PackageManager,
    command: &str,
    install_args: Vec<String>,
  ) -> Result<()> {
    let mut cmd = Command::new(package_manager.to_str());
    cmd.arg(command).args(install_args);

    if self.flags.global {
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
  use tempfile::tempdir;

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
  fn test_detect_package_manager_from_json() {
    let package_json = PackageJson {
      package_manager: Some("pnpm@9.10.0".to_owned()),
      ..Default::default()
    };

    let manager = PackageJsonManager {
      json: package_json,
      ..Default::default()
    };

    assert_eq!(manager.detect_package_manager(), PackageManager::Pnpm);
  }

  #[test]
  fn test_detect_package_manager_from_lock_file() {
    let dir = tempdir().unwrap();
    let lock_file = dir.path().join("pnpm-lock.yaml");
    std::fs::write(&lock_file, "").unwrap();

    let package_json = PackageJson {
      package_manager: None,
      ..Default::default()
    };

    let manager = PackageJsonManager {
      file_path: Some(dir.path().join("package.json")),
      json: package_json,
      ..Default::default()
    };

    assert_eq!(manager.detect_package_manager(), PackageManager::Pnpm);
  }
}
