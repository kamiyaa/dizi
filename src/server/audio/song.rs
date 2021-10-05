use std::io;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    #[serde(rename = "path")]
    _path: PathBuf,
    #[serde(rename = "metadata", default)]
    _metadata: String,
}

impl Song {
    pub fn new(p: &Path) -> io::Result<Self> {
        Ok(Self {
            _path: p.to_path_buf(),
            _metadata: "".to_string(),
        })
    }

    pub fn file_path(&self) -> &Path {
        self._path.as_path()
    }

    pub fn metadata(&self) -> &String {
        &self._metadata
    }
}
