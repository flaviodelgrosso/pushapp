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
