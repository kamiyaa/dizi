use std::fs;

use crate::config::option::SortOption;

#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub _show_hidden: bool,
    pub _show_icons: bool,
    pub _sort_options: SortOption,
    pub _scroll_offset: usize,
}

impl DisplayOption {
    pub fn show_hidden(&self) -> bool {
        self._show_hidden
    }

    pub fn set_show_hidden(&mut self, show_hidden: bool) {
        self._show_hidden = show_hidden;
    }

    pub fn scroll_offset(&self) -> usize {
        self._scroll_offset
    }

    pub fn show_icons(&self) -> bool {
        self._show_icons
    }

    pub fn sort_options_ref(&self) -> &SortOption {
        &self._sort_options
    }

    pub fn sort_options_mut(&mut self) -> &mut SortOption {
        &mut self._sort_options
    }

    pub fn filter_func(&self) -> fn(&Result<fs::DirEntry, std::io::Error>) -> bool {
        if self.show_hidden() {
            no_filter
        } else {
            filter_hidden
        }
    }
}

impl std::default::Default for DisplayOption {
    fn default() -> Self {
        Self {
            _show_hidden: false,
            _show_icons: false,
            _sort_options: SortOption::default(),
            _scroll_offset: 4,
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
