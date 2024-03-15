use serde_derive::Deserialize;
use std::convert::From;

use crate::config::option::LayoutComposition;
use crate::config::{parse_json_to_config, JsonConfigFile};

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum LayoutCompositionRaw {
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
pub struct AppLayoutRaw {
    pub layout: LayoutCompositionRaw,
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

impl From<AppLayoutRaw> for AppLayout {
    fn from(raw: AppLayoutRaw) -> Self {
        let res = LayoutComposition::from(&raw.layout);

        let layout = res.unwrap_or_else(|_| LayoutComposition::default());
        Self { layout }
    }
}

impl JsonConfigFile for AppLayout {
    fn get_config(file_name: &str) -> Self {
        match parse_json_to_config::<AppLayoutRaw, AppLayout>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse layout config: {}", e);
                Self::default()
            }
        }
    }
}
