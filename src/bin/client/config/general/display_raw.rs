use std::convert::From;

use serde::Deserialize;

use crate::config::option::DisplayOption;

use super::file_sort_option_raw::FileSortConfigRaw;

const fn default_scroll_offset() -> usize {
    4
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisplayOptionRaw {
    #[serde(default = "default_scroll_offset")]
    scroll_offset: usize,

    #[serde(default)]
    show_hidden: bool,

    #[serde(default)]
    show_icons: bool,

    #[serde(default, rename = "sort")]
    sort_options: FileSortConfigRaw,
}

impl From<DisplayOptionRaw> for DisplayOption {
    fn from(raw: DisplayOptionRaw) -> Self {
        Self {
            show_hidden: raw.show_hidden,
            show_icons: raw.show_icons,
            sort_options: raw.sort_options.into(),
            scroll_offset: raw.scroll_offset,
        }
    }
}

impl std::default::Default for DisplayOptionRaw {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
            sort_options: FileSortConfigRaw::default(),
            scroll_offset: default_scroll_offset(),
        }
    }
}
