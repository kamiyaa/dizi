use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::DisplayOption;

use super::sort_raw::SortOptionRaw;

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
    sort_options: SortOptionRaw,
}

impl From<DisplayOptionRaw> for DisplayOption {
    fn from(raw: DisplayOptionRaw) -> Self {
        Self {
            _show_hidden: raw.show_hidden,
            _show_icons: raw.show_icons,
            _sort_options: raw.sort_options.into(),
            _scroll_offset: raw.scroll_offset,
        }
    }
}

impl std::default::Default for DisplayOptionRaw {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
            sort_options: SortOptionRaw::default(),
            scroll_offset: default_scroll_offset(),
        }
    }
}
