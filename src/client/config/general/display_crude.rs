use std::convert::From;

use serde_derive::Deserialize;

use crate::config::option::DisplayOption;

use super::sort_crude::SortOptionCrude;

const fn default_scroll_offset() -> usize {
    4
}

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisplayOptionCrude {
    #[serde(default = "default_scroll_offset")]
    scroll_offset: usize,

    #[serde(default)]
    show_hidden: bool,

    #[serde(default)]
    show_icons: bool,

    #[serde(default, rename = "sort")]
    sort_options: SortOptionCrude,
}

impl From<DisplayOptionCrude> for DisplayOption {
    fn from(crude: DisplayOptionCrude) -> Self {
        Self {
            _show_hidden: crude.show_hidden,
            _show_icons: crude.show_icons,
            _sort_options: crude.sort_options.into(),
            _scroll_offset: crude.scroll_offset,
        }
    }
}

impl std::default::Default for DisplayOptionCrude {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
            sort_options: SortOptionCrude::default(),
            scroll_offset: default_scroll_offset(),
        }
    }
}
