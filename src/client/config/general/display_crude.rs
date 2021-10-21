use std::convert::From;
use std::path;

use serde_derive::Deserialize;
use serde_json::Value;

use tui::layout::Constraint;

use crate::config::option::DisplayOption;

use super::sort_crude::SortOptionCrude;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisplayOptionCrude {
    #[serde(default)]
    show_hidden: bool,

    #[serde(default, rename = "sort")]
    sort_options: SortOptionCrude,
}

impl From<DisplayOptionCrude> for DisplayOption {
    fn from(crude: DisplayOptionCrude) -> Self {
        Self {
            _show_hidden: crude.show_hidden,
            _sort_options: crude.sort_options.into(),
        }
    }
}

impl std::default::Default for DisplayOptionCrude {
    fn default() -> Self {
        Self {
            show_hidden: false,
            sort_options: SortOptionCrude::default(),
        }
    }
}
