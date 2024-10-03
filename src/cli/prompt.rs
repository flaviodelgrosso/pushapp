use inquire::{formatter::MultiOptionFormatter, MultiSelect};

use super::package_info::PackageInfo;

pub fn display_update(updatable_packages: Vec<PackageInfo>) -> Option<Vec<PackageInfo>> {
  let formatter: MultiOptionFormatter<'_, PackageInfo> =
    &|a| format!("{} package(s) selected", a.len());

  let prompt_message = format!(
    "Choose packages to update ({} total):",
    updatable_packages.len()
  );

  let prompt = MultiSelect::new(&prompt_message, updatable_packages)
    .with_formatter(&formatter)
    .prompt();

  match prompt {
    Ok(selected) => Some(selected),
    Err(_) => None,
  }
}
