use std::convert::From;
use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::config::option::DisplayOption;

use super::display_crude::DisplayOptionCrude;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct ClientConfigCrude {
    #[serde(default)]
    pub socket: String,
    #[serde(default)]
    pub home_dir: String,

    #[serde(default, rename = "display")]
    pub display_options: DisplayOptionCrude,
}

impl std::default::Default for ClientConfigCrude {
    fn default() -> Self {
        Self {
            socket: "".to_string(),
            home_dir: "".to_string(),
            display_options: DisplayOptionCrude::default(),
        }
    }
}

impl From<ClientConfigCrude> for ClientConfig {
    fn from(crude: ClientConfigCrude) -> Self {
        Self {
            socket: PathBuf::from(crude.socket),
            home_dir: PathBuf::from(crude.home_dir),
            display_options: DisplayOption::from(crude.display_options),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub socket: PathBuf,
    pub home_dir: PathBuf,
    pub display_options: DisplayOption,
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
        }
    }
}
