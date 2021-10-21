use serde_derive::Deserialize;
use std::convert::From;
use std::str::FromStr;

use tui::layout::Direction;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::config::option::{LayoutComposition, WidgetType};
use crate::config::{parse_json_to_config, JsonConfigFile};

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
        widgets: Vec<Self>,
        ratio: usize,
    },
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
        let layout = LayoutComposition::default();
        Self { layout }
    }
}

impl From<AppLayoutCrude> for AppLayout {
    fn from(crude: AppLayoutCrude) -> Self {
        let res = LayoutComposition::from(&crude.layout);

        let layout = res.unwrap_or_else(|_| LayoutComposition::default());
        Self { layout }
    }
}

impl JsonConfigFile for AppLayout {
    fn get_config(file_name: &str) -> Self {
        parse_json_to_config::<AppLayoutCrude, AppLayout>(file_name).unwrap_or_else(Self::default)
    }
}
