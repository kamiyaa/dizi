use std::convert::From;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use shellexpand::tilde_with_context;

use crate::config::option::DisplayOption;

use super::display_raw::DisplayOptionRaw;

#[derive(Clone, Debug, Deserialize)]
pub struct ClientConfigRaw {
    #[serde(default)]
    pub socket: String,
    #[serde(default)]
    pub home_dir: Option<String>,

    #[serde(default, rename = "display")]
    pub display_options: DisplayOptionRaw,
}

impl std::default::Default for ClientConfigRaw {
    fn default() -> Self {
        Self {
            socket: "".to_string(),
            home_dir: None,
            display_options: DisplayOptionRaw::default(),
        }
    }
}

impl From<ClientConfigRaw> for ClientConfig {
    fn from(raw: ClientConfigRaw) -> Self {
        let socket = PathBuf::from(tilde_with_context(&raw.socket, dirs_next::home_dir).as_ref());
        let home_dir = raw.home_dir.map(|home_dir| {
            PathBuf::from(tilde_with_context(&home_dir, dirs_next::home_dir).as_ref())
        });

        Self {
            socket,
            home_dir,
            display_options: DisplayOption::from(raw.display_options),
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
    pub fn socket_ref(&self) -> &Path {
        self.socket.as_path()
    }
    pub fn display_options_ref(&self) -> &DisplayOption {
        &self.display_options
    }
}

impl std::default::Default for ClientConfig {
    fn default() -> Self {
        let socket =
            PathBuf::from(tilde_with_context("~/dizi-server-socket", dirs_next::home_dir).as_ref());

        Self {
            socket,
            home_dir: None,
            display_options: DisplayOption::default(),
        }
    }
}
