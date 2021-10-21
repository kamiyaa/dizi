use std::fs;

use tui::layout::Constraint;

use crate::config::option::SortOption;

pub const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

#[derive(Clone, Debug)]
pub struct DisplayOption {
    pub column_ratio: (usize, usize, usize),
    pub _show_borders: bool,
    pub _show_hidden: bool,
    pub _sort_options: SortOption,
    pub default_layout: [Constraint; 3],
    pub no_preview_layout: [Constraint; 3],
}

impl DisplayOption {
    pub fn show_borders(&self) -> bool {
        self._show_borders
    }

    pub fn show_hidden(&self) -> bool {
        self._show_hidden
    }

    pub fn set_show_hidden(&mut self, show_hidden: bool) {
        self._show_hidden = show_hidden;
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
        let column_ratio = default_column_ratio();

        let total = (column_ratio.0 + column_ratio.1 + column_ratio.2) as u32;
        let default_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32, total),
            Constraint::Ratio(column_ratio.2 as u32, total),
        ];
        let no_preview_layout = [
            Constraint::Ratio(column_ratio.0 as u32, total),
            Constraint::Ratio(column_ratio.1 as u32 + column_ratio.2 as u32, total),
            Constraint::Ratio(0, total),
        ];

        Self {
            column_ratio,
            _show_borders: true,
            _show_hidden: false,
            _sort_options: SortOption::default(),
            default_layout,
            no_preview_layout,
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
