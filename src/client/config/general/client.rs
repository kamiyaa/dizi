use std::convert::From;
use std::path::PathBuf;

use serde_derive::Deserialize;
use shellexpand::tilde_with_context;

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
    pub home_dir: Option<String>,

    #[serde(default, rename = "display")]
    pub display_options: DisplayOptionCrude,
}

impl std::default::Default for ClientConfigCrude {
    fn default() -> Self {
        Self {
            socket: "".to_string(),
            home_dir: None,
            display_options: DisplayOptionCrude::default(),
        }
    }
}

impl From<ClientConfigCrude> for ClientConfig {
    fn from(crude: ClientConfigCrude) -> Self {

        let socket = PathBuf::from(tilde_with_context(&crude.socket, dirs_next::home_dir).as_ref());
        let home_dir = if let Some(home_dir) = crude.home_dir {
            Some(PathBuf::from(tilde_with_context(&home_dir, dirs_next::home_dir).as_ref()))
        } else {
            None
        };

        Self {
            socket,
            home_dir,
            display_options: DisplayOption::from(crude.display_options),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub socket: PathBuf,
    pub home_dir: Option<PathBuf>,
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
            socket: PathBuf::from("/tmp/dizi-server-socket"),
            home_dir: None,
            display_options: DisplayOption::default(),
        }
    }
}
