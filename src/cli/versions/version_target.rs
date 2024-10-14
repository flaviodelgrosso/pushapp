use core::fmt;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum VersionTarget {
  #[default]
  Latest,
  Semver,
  Major,
  Minor,
  Patch,
  Pre,
  Next,
  Canary,
  Rc,
  Beta,
  Alpha,
}

impl fmt::Display for VersionTarget {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      VersionTarget::Latest => write!(f, "latest"),
      VersionTarget::Semver => write!(f, "semver"),
      VersionTarget::Major => write!(f, "major"),
      VersionTarget::Minor => write!(f, "minor"),
      VersionTarget::Patch => write!(f, "patch"),
      VersionTarget::Pre => write!(f, "pre"),
      VersionTarget::Next => write!(f, "next"),
      VersionTarget::Canary => write!(f, "canary"),
      VersionTarget::Rc => write!(f, "rc"),
      VersionTarget::Beta => write!(f, "beta"),
      VersionTarget::Alpha => write!(f, "alpha"),
    }
  }
}
