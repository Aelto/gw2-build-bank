use std::fmt::write;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use crate::constants;

use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct Storage {
  builds: Vec<Build>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
  pub template: String,
  pub name: String,
  pub profession: String
}

impl Build {
  pub fn new(template: &str, name: &str, profession: &str) -> Self {
    Build {
      template: template.to_owned(),
      name: template.to_owned(),
      profession: profession.to_owned()
    }
  }

  /// returns true if the build was inserted in the storage and false if it
  /// was not.
  pub fn insert(self) -> io::Result<bool> {
    if Self::exists(&self.name).unwrap_or(false) {
      return Ok(false);
    }

    let builds = Self::all()
      .unwrap_or_else(|_| Vec::new());
    
    let mut storage = Storage { builds };
    storage.builds.push(self);

    let path = Path::new(constants::BANK_PATH)
      .join(constants::BANK_NAME);

    let content = toml::to_string_pretty(&storage)
      .map_err(|_err| std::io::ErrorKind::InvalidData)?;

    let mut file = fs::File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(true)
  }

  pub fn all() -> io::Result<Vec<Build>> {
    let path = Path::new(constants::BANK_PATH)
      .join(constants::BANK_NAME);

    let content = fs::read_to_string(path)?;

    let storage: Storage = toml::from_str(&content)?;

    Ok(storage.builds)
  }

  pub fn exists(name: &str) -> io::Result<bool> {
    let already_contains_name = Self::all()?
    .iter()
    .any(|build| build.name == name);

    Ok(already_contains_name)
  }

  pub fn remove(name: &str) -> io::Result<()> {
    let builds = Self::all()
      .unwrap_or_else(|_| Vec::new())
      .into_iter()
      .filter(|build| build.name != name)
      .collect();
    
    let mut storage = Storage { builds };

    let path = Path::new(constants::BANK_PATH)
      .join(constants::BANK_NAME);

    let content = toml::to_string_pretty(&storage)
      .map_err(|_err| std::io::ErrorKind::InvalidData)?;

    let mut file = fs::File::create(&path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
  }
}