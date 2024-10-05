use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum PackageManager {
  Npm,
  Yarn,
  Pnpm,
  Bun,
}

const NPM: &str = "npm";
const YARN: &str = "yarn";
const PNPM: &str = "pnpm";
const BUN: &str = "bun";

pub const NPM_LOCK: &str = "package-lock.json";
pub const YARN_LOCK: &str = "yarn.lock";
pub const PNPM_LOCK: &str = "pnpm-lock.yaml";
pub const BUN_LOCK: &str = "bun.lockb";

impl PackageManager {
  pub fn from_lock_file(lock_file: &str) -> Self {
    match lock_file {
      NPM_LOCK => Self::Npm,
      YARN_LOCK => Self::Yarn,
      PNPM_LOCK => Self::Pnpm,
      BUN_LOCK => Self::Bun,
      _ => unreachable!(),
    }
  }

  pub fn to_str(&self) -> &'static str {
    match self {
      Self::Npm => NPM,
      Self::Yarn => YARN,
      Self::Pnpm => PNPM,
      Self::Bun => BUN,
    }
  }

  pub fn determine_install_command(&self) -> &str {
    match self {
      PackageManager::Npm => "install",
      _ => "add",
    }
  }
}

impl Display for PackageManager {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_str())
  }
}

impl From<&str> for PackageManager {
  fn from(s: &str) -> Self {
    match s {
      NPM => PackageManager::Npm,
      YARN => PackageManager::Yarn,
      PNPM => PackageManager::Pnpm,
      BUN => PackageManager::Bun,
      _ => unreachable!(),
    }
  }
}
