use serde_derive::Deserialize;
use std::collections::HashMap;

use dizi::error::DiziResult;

use super::DEFAULT_CONFIG_FILE_PATH;
use super::{AppStyle, AppStyleRaw};
use crate::config::{parse_toml_to_config, TomlConfigFile};

#[derive(Clone, Debug, Default, Deserialize)]
pub struct AppThemeRaw {
    #[serde(default)]
    pub playing: AppStyleRaw,
    #[serde(default)]
    pub playlist: AppStyleRaw,

    #[serde(default)]
    pub regular: AppStyleRaw,
    #[serde(default)]
    pub directory: AppStyleRaw,
    #[serde(default)]
    pub executable: AppStyleRaw,
    #[serde(default)]
    pub link: AppStyleRaw,
    #[serde(default)]
    pub link_invalid: AppStyleRaw,
    #[serde(default)]
    pub socket: AppStyleRaw,
    #[serde(default)]
    pub ext: HashMap<String, AppStyleRaw>,
}

#[derive(Clone, Debug)]
pub struct AppTheme {
    pub playing: AppStyle,
    pub playlist: AppStyle,

    pub regular: AppStyle,
    pub directory: AppStyle,
    pub executable: AppStyle,
    pub link: AppStyle,
    pub link_invalid: AppStyle,
    pub socket: AppStyle,
    pub ext: HashMap<String, AppStyle>,
}

impl From<AppThemeRaw> for AppTheme {
    fn from(raw: AppThemeRaw) -> Self {
        let playing = raw.playing.to_style_theme();
        let playlist = raw.playlist.to_style_theme();

        let executable = raw.executable.to_style_theme();
        let regular = raw.regular.to_style_theme();
        let directory = raw.directory.to_style_theme();
        let link = raw.link.to_style_theme();
        let link_invalid = raw.link_invalid.to_style_theme();
        let socket = raw.socket.to_style_theme();
        let ext: HashMap<String, AppStyle> = raw
            .ext
            .iter()
            .map(|(k, v)| {
                let style = v.to_style_theme();
                (k.clone(), style)
            })
            .collect();

        Self {
            playing,
            playlist,

            executable,
            regular,
            directory,
            link,
            link_invalid,
            socket,
            ext,
        }
    }
}

impl AppTheme {
    pub fn default_res() -> DiziResult<Self> {
        let raw: AppThemeRaw = toml::from_str(DEFAULT_CONFIG_FILE_PATH)?;
        Ok(Self::from(raw))
    }
}

impl TomlConfigFile for AppTheme {
    fn get_config(file_name: &str) -> Self {
        match parse_toml_to_config::<AppThemeRaw, AppTheme>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse theme config: {}", e);
                Self::default()
            }
        }
    }
}

impl std::default::Default for AppTheme {
    fn default() -> Self {
        // This should not fail.
        // If it fails then there is a (syntax) error in the default config file
        Self::default_res().unwrap()
    }
}
