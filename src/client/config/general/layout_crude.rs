use serde_derive::Deserialize;
use std::convert::From;
use std::str::FromStr;

use tui::layout::Direction;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::config::option::{LayoutComposition, WidgetType};
use crate::config::{parse_to_config_file, TomlConfigFile};

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppLayoutCrude {
    pub layout: LayoutCompositionCrude,
}

#[derive(Clone, Debug)]
pub struct AppLayout {
    pub layout: LayoutComposition,
}

impl std::default::Default for AppLayout {
    fn default() -> Self {
        let layout = LayoutComposition::Composite {
            direction: Direction::Horizontal,
            ratio: 1,
            widgets: vec![
                LayoutComposition::Simple {
                    widget: WidgetType::FileBrowser,
                    ratio: 1,
                    border: true,
                    title: true,
                },
                LayoutComposition::Composite {
                    direction: Direction::Vertical,
                    ratio: 1,
                    widgets: vec![
                        LayoutComposition::Simple {
                            widget: WidgetType::MusicPlayer,
                            ratio: 1,
                            border: true,
                            title: true,
                        },
                        LayoutComposition::Simple {
                            widget: WidgetType::Playlist,
                            ratio: 1,
                            border: true,
                            title: true,
                        },
                    ],
                },
            ],
        };

        Self { layout }
    }
}

impl From<AppLayoutCrude> for AppLayout {
    fn from(crude: AppLayoutCrude) -> Self {
        Self {
            layout: LayoutComposition::from(&crude.layout).unwrap(),
        }
    }
}

impl TomlConfigFile for AppLayout {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppLayoutCrude, AppLayout>(file_name).unwrap_or_else(Self::default)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum LayoutCompositionCrude {
    #[serde(rename = "simple")]
    Simple {
        widget: String,
        ratio: usize,
        #[serde(default)]
        border: bool,
        #[serde(default)]
        title: bool,
    },
    #[serde(rename = "composite")]
    Composite {
        direction: String,
        widgets: Vec<LayoutCompositionCrude>,
        ratio: usize,
    },
}
