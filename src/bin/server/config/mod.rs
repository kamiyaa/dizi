pub mod general;

pub use self::general::*;

use dizi::error::{AppError, AppErrorCode, AppResult};
use serde::de::DeserializeOwned;
use std::fs;
use std::path::{Path, PathBuf};

use crate::CONFIG_HIERARCHY;

pub trait TomlConfigFile {
    fn get_config(file_name: &str) -> Self;
}

// searches a list of folders for a given file in order of preference
pub fn search_directories<P>(filename: &str, directories: &[P]) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    for path in directories.iter() {
        let filepath = path.as_ref().join(filename);
        if filepath.exists() {
            return Some(filepath);
        }
    }
    None
}

// parses a config file into its appropriate format
fn parse_toml_to_config<T, S>(filename: &str) -> AppResult<S>
where
    T: DeserializeOwned,
    S: From<T>,
{
    match search_directories(filename, &CONFIG_HIERARCHY) {
        Some(file_path) => {
            let file_contents = fs::read_to_string(&file_path)?;
            let config = toml::from_str::<T>(&file_contents)?;
            Ok(S::from(config))
        }
        None => {
            let error = AppError::new(AppErrorCode::Io, "No config directory found".to_string());
            Err(error)
        }
    }
}
