use colored::Colorize;
use std::fmt::Display;

use semver::Version;

#[derive(Debug)]
pub struct PackageInfo {
  pub pkg_name: String,
  pub current_version: String,
  pub latest_version: String,
}

impl Display for PackageInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let current_ver = Version::parse(normalize_version(&self.current_version)).unwrap();
    let latest_ver = Version::parse(normalize_version(&self.latest_version)).unwrap();

    let color = if latest_ver.major > current_ver.major {
      "red"
    } else if latest_ver.minor > current_ver.minor {
      "blue"
    } else {
      "green"
    };

    let colored_latest_version = match color {
      "red" => self.latest_version.bright_red().bold(),
      "blue" => self.latest_version.bright_yellow().bold(),
      "green" => self.latest_version.bright_green().bold(),
      _ => unreachable!(),
    };

    write!(
      f,
      "{}: {} → {}",
      self.pkg_name, self.current_version, colored_latest_version
    )
  }
}

pub fn normalize_version(version: &str) -> &str {
  version.trim_start_matches(&['^', '~'][..])
}
