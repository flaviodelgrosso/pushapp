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
    // Parse and normalize versions
    let current_ver_result = Version::parse(normalize_version(&self.current_version));
    let latest_ver_result = Version::parse(normalize_version(&self.latest_version));

    // Handle parsing errors
    let Ok(current_ver) = current_ver_result else {
      return Err(std::fmt::Error);
    };

    let Ok(latest_ver) = latest_ver_result else {
      return Err(std::fmt::Error);
    };

    // Determine color based on version comparison
    let colored_latest_version = if latest_ver.major > current_ver.major {
      self.latest_version.bright_red().bold()
    } else if latest_ver.minor > current_ver.minor {
      self.latest_version.bright_yellow().bold()
    } else {
      self.latest_version.bright_green().bold()
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
