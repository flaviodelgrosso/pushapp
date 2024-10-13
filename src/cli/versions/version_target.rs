use core::fmt;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum VersionTarget {
  #[default]
  Latest,
  Major,
  Minor,
  Patch,
  Pre,
}

impl fmt::Display for VersionTarget {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      VersionTarget::Latest => write!(f, "latest"),
      VersionTarget::Major => write!(f, "major"),
      VersionTarget::Minor => write!(f, "minor"),
      VersionTarget::Patch => write!(f, "patch"),
      VersionTarget::Pre => write!(f, "pre"),
    }
  }
}
