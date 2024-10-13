use core::fmt;

use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum VersionTarget {
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
