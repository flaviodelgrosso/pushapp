pub fn normalize_version(version: &str) -> &str {
  version.trim_start_matches(|c: char| !c.is_numeric())
}
