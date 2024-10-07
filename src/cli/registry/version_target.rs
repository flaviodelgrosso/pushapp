use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum VersionTarget {
  Latest,
  Minor,
  Patch,
  Newest,
}

impl VersionTarget {
  pub fn to_str(&self) -> &'static str {
    match self {
      Self::Latest => "latest",
      Self::Minor => "minor",
      Self::Patch => "patch",
      Self::Newest => "newest",
    }
  }
}

impl FromStr for VersionTarget {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "latest" => Ok(VersionTarget::Latest),
      "minor" => Ok(VersionTarget::Minor),
      "patch" => Ok(VersionTarget::Patch),
      "newest" => Ok(VersionTarget::Newest),
      _ => Err(format!("Invalid version target: {s}")),
    }
  }
}
