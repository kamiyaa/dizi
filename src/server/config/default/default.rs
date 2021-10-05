use std::path;

use serde_derive::Deserialize;

use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};

#[derive(Clone, Debug, Deserialize)]
pub struct RawServerConfig {
    #[serde(default)]
    pub socket: path::PathBuf,
}

impl Flattenable<ServerConfig> for RawServerConfig {
    fn flatten(self) -> ServerConfig {
        ServerConfig {
            socket: path::PathBuf::from(self.socket),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    #[serde(default)]
    pub socket: path::PathBuf,
}

impl std::default::Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket: path::PathBuf::from("."),
        }
    }
}

impl ConfigStructure for ServerConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawServerConfig, ServerConfig>(file_name).unwrap_or_else(Self::default)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawAppConfig {
    #[serde(default, rename = "server")]
    _server: ServerConfig,
}

impl Flattenable<AppConfig> for RawAppConfig {
    fn flatten(self) -> AppConfig {
        AppConfig {
            _server: self._server,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    _server: ServerConfig,
}

impl AppConfig {
    pub fn new(server: ServerConfig) -> Self {
        Self {
            _server: server,
        }
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
