use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::config::Flattenable;
use crate::util::display_option::DisplayOption;

use super::display::RawDisplayOption;
use super::player::{PlayerOption, RawPlayerOption};

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawClientConfig {
    #[serde(default)]
    pub socket: String,
    #[serde(default)]
    pub home_dir: String,

    #[serde(default, rename = "display")]
    pub display_options: RawDisplayOption,
    #[serde(default, rename = "player")]
    pub player_options: RawPlayerOption,
}

impl Flattenable<ClientConfig> for RawClientConfig {
    fn flatten(self) -> ClientConfig {
        ClientConfig {
            socket: PathBuf::from(self.socket),
            home_dir: PathBuf::from(self.home_dir),
            display_options: self.display_options.flatten(),
            player_options: self.player_options.flatten(),
        }
    }
}

impl std::default::Default for RawClientConfig {
    fn default() -> Self {
        Self {
            socket: "".to_string(),
            home_dir: "".to_string(),
            display_options: RawDisplayOption::default(),
            player_options: RawPlayerOption::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub socket: PathBuf,
    pub home_dir: PathBuf,
    pub display_options: DisplayOption,
    pub player_options: PlayerOption,
}

impl ClientConfig {
    pub fn display_options_ref(&self) -> &DisplayOption {
        &self.display_options
    }
}

impl std::default::Default for ClientConfig {
    fn default() -> Self {
        Self {
            socket: PathBuf::from(""),
            home_dir: PathBuf::from(""),
            display_options: DisplayOption::default(),
            player_options: PlayerOption::default(),
        }
    }
}
