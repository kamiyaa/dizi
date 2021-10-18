use serde_derive::Deserialize;

use super::server::{RawServerConfig, ServerConfig};
use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppConfig {
    #[serde(default, rename = "server")]
    _server: RawServerConfig,
}

impl Flattenable<AppConfig> for RawAppConfig {
    fn flatten(self) -> AppConfig {
        AppConfig {
            _server: self._server.flatten(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    _server: ServerConfig,
}

impl AppConfig {
    pub fn new(server: ServerConfig) -> Self {
        Self { _server: server }
    }

    pub fn server_ref(&self) -> &ServerConfig {
        &self._server
    }
}

impl ConfigStructure for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawAppConfig, AppConfig>(file_name).unwrap_or_else(Self::default)
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            _server: ServerConfig::default(),
        }
    }
}
