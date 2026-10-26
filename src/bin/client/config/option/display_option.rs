use std::fs;

use crate::config::option::FileSortConfig;

#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub show_hidden: bool,
    pub show_icons: bool,
    pub sort_options: FileSortConfig,
    pub scroll_offset: usize,
}

impl DisplayOption {
    pub fn filter_func(&self) -> fn(&Result<fs::DirEntry, std::io::Error>) -> bool {
        if self.show_hidden {
            no_filter
        } else {
            filter_hidden
        }
    }
}

impl std::default::Default for DisplayOption {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
            sort_options: FileSortConfig::default(),
            scroll_offset: 4,
        }
    }
}

const fn no_filter(_: &Result<fs::DirEntry, std::io::Error>) -> bool {
    true
}

fn filter_hidden(result: &Result<fs::DirEntry, std::io::Error>) -> bool {
    match result {
        Err(_) => true,
        Ok(entry) => {
            let file_name = entry.file_name();
            let lossy_string = file_name.as_os_str().to_string_lossy();
            !lossy_string.starts_with('.')
        }
    }
}
