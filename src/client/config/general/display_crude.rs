use std::convert::From;
use std::path;

use serde_derive::Deserialize;
use serde_json::Value;

use tui::layout::Constraint;

use crate::config::option::DisplayOption;

use super::sort_crude::SortOptionCrude;

const fn default_column_ratio() -> (usize, usize, usize) {
    (1, 3, 4)
}

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisplayOptionCrude {
    #[serde(default)]
    show_hidden: bool,

    #[serde(default = "default_true")]
    show_borders: bool,

    #[serde(default)]
    column_ratio: Option<[usize; 3]>,

    #[serde(default)]
    layout: String,

    #[serde(default, rename = "sort")]
    sort_options: SortOptionCrude,
}

impl From<DisplayOptionCrude> for DisplayOption {
    fn from(crude: DisplayOptionCrude) -> Self {
        let column_ratio = match crude.column_ratio {
            Some(s) => (s[0], s[1], s[2]),
            _ => default_column_ratio(),
        };

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

        /*
        let file = {
            let tilde_cow = shellexpand::tilde_with_context(self.layout, dirs_next::home_dir);
            let tilde_path = path::PathBuf::from(tilde_cow.as_ref());
            File::Open(tilde_path)
        };

        let json = serde_json::from_str();
        let layout =

        */

        //        let layout = "".to_string();

        Self {
            column_ratio,
            _show_hidden: crude.show_hidden,
            _show_borders: crude.show_borders,
            _sort_options: crude.sort_options.into(),
            //            layout,
            default_layout,
            no_preview_layout,
        }
    }
}

impl std::default::Default for DisplayOptionCrude {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_borders: true,
            column_ratio: None,
            layout: "".to_string(),
            sort_options: SortOptionCrude::default(),
        }
    }
}
