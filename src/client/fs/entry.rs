use std::{fs, io, path};

use crate::fs::{FileType, Metadata};

use crate::util::display_option::DisplayOption;

#[derive(Clone, Debug)]
pub struct DirEntry {
    name: String,
    label: String,
    path: path::PathBuf,
    pub metadata: Metadata,
    selected: bool,
    marked: bool,
}

impl DirEntry {
    pub fn from(direntry: &fs::DirEntry, options: &DisplayOption) -> io::Result<Self> {
        let path = direntry.path();

        let mut metadata = Metadata::from(&path)?;
        let name = direntry
            .file_name()
            .as_os_str()
            .to_string_lossy()
            .to_string();

        let label = name.clone();

        Ok(Self {
            name,
            label,
            path,
            metadata,
            selected: false,
            marked: false,
        })
    }

    pub fn update_label(&mut self, label: String) {
        self.label = label;
    }

    pub fn file_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub fn file_path(&self) -> &path::Path {
        self.path.as_path()
    }

    pub fn is_selected(&self) -> bool {
        self.selected
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    pub fn get_ext(&self) -> &str {
        let fname = self.file_name();
        match fname.rfind('.') {
            Some(pos) => &fname[pos..],
            None => "",
        }
    }
}

impl std::fmt::Display for DirEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.file_name())
    }
}

impl std::convert::AsRef<str> for DirEntry {
    fn as_ref(&self) -> &str {
        self.file_name()
    }
}

impl std::cmp::PartialEq for DirEntry {
    fn eq(&self, other: &Self) -> bool {
        self.file_path() == other.file_path()
    }
}
impl std::cmp::Eq for DirEntry {}

impl std::cmp::PartialOrd for DirEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for DirEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.file_path().cmp(other.file_path())
    }
}
