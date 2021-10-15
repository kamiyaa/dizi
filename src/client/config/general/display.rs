use std::path;

use serde_derive::Deserialize;
use serde_json::Value;

use tui::layout::Constraint;

use crate::config::Flattenable;
use crate::util::display_option::{default_column_ratio, DisplayOption};

use super::SortRawOption;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawDisplayOption {
    #[serde(default)]
    show_hidden: bool,

    #[serde(default = "default_true")]
    show_borders: bool,

    #[serde(default)]
    column_ratio: Option<[usize; 3]>,

    #[serde(default)]
    layout: String,

    #[serde(default, rename = "sort")]
    sort_options: SortRawOption,
}

impl Flattenable<DisplayOption> for RawDisplayOption {
    fn flatten(self) -> DisplayOption {
        let column_ratio = match self.column_ratio {
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

        DisplayOption {
            column_ratio,
            _show_hidden: self.show_hidden,
            _show_borders: self.show_borders,
            _sort_options: self.sort_options.into(),
            //            layout,
            default_layout,
            no_preview_layout,
        }
    }
}

impl std::default::Default for RawDisplayOption {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_borders: true,
            column_ratio: None,
            layout: "".to_string(),
            sort_options: SortRawOption::default(),
        }
    }
}
