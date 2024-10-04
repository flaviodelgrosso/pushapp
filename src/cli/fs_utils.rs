use anyhow::{format_err, Result};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn find_closest_file<P: AsRef<Path>>(filename: &str, current_dir: P) -> Result<PathBuf> {
  let mut current_dir = PathBuf::from(current_dir.as_ref());
  loop {
    let file_path = current_dir.join(filename);
    if file_path.exists() {
      return Ok(file_path);
    }
    if !current_dir.pop() {
      return Err(format_err!(
        "Couldn't find an available \"{}\" from {}.",
        filename,
        current_dir.display()
      ));
    }
  }
}

pub fn read_json<Json, FilePath>(file_path: FilePath) -> Result<Json>
where
  Json: serde::de::DeserializeOwned,
  FilePath: AsRef<Path>,
{
  let mut file = File::open(file_path)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  let serialized_json = serde_json::from_str(&contents);

  match serialized_json {
    Ok(json) => Ok(json),
    Err(error) => Err(format_err!(error)),
  }
}

#[cfg(test)]
mod tests {
  use super::super::package_json::PACKAGE_JSON_FILENAME;
  use super::*;
  use std::io::Write;
  use tempfile::tempdir;

  #[test]
  fn test_find_closest_file() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join(PACKAGE_JSON_FILENAME);
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{{\"name\": \"test\"}}").unwrap();

    let found_file = find_closest_file(PACKAGE_JSON_FILENAME, dir.path()).unwrap();
    assert_eq!(found_file, file_path);
  }

  #[test]
  fn test_find_closest_file_not_found() {
    let dir = tempdir().unwrap();
    let result = find_closest_file("non_existent_file.json", dir.path());
    assert!(result.is_err());
  }

  #[test]
  fn test_read_json() {
    #[derive(serde::Deserialize, Debug, PartialEq)]
    struct PackageJson {
      name: String,
    }

    let dir = tempdir().unwrap();
    let file_path = dir.path().join(PACKAGE_JSON_FILENAME);
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "{{\"name\": \"test\"}}").unwrap();

    let json: PackageJson = read_json(file_path).unwrap();
    assert_eq!(
      json,
      PackageJson {
        name: "test".to_string()
      }
    );
  }

  #[test]
  fn test_read_json_invalid_format() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join(PACKAGE_JSON_FILENAME);
    let mut file = File::create(&file_path).unwrap();
    writeln!(file, "invalid json").unwrap();

    let result: Result<serde_json::Value> = read_json(file_path);
    assert!(result.is_err());
  }
}
