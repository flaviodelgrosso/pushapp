use std::fmt::Display;

#[derive(Debug, Default)]
pub enum PackageLock {
  #[default]
  Npm,
  Yarn,
  Pnpm,
  Bun,
}

impl Display for PackageLock {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let package_manager = match self {
      PackageLock::Npm => "npm",
      PackageLock::Yarn => "yarn",
      PackageLock::Pnpm => "pnpm",
      PackageLock::Bun => "bun",
    };

    write!(f, "{package_manager}")
  }
}
