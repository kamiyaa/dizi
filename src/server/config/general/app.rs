use serde_derive::Deserialize;

use crate::config::{parse_toml_to_config, TomlConfigFile};

use super::{ServerConfig, ServerConfigRaw};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigCrude {
    #[serde(default)]
    pub server: ServerConfigRaw,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    server: ServerConfig,
}

impl From<AppConfigCrude> for AppConfig {
    fn from(crude: AppConfigCrude) -> Self {
        Self {
            server: ServerConfig::from(crude.server),
        }
    }
}

impl AppConfig {
    pub fn new(server: ServerConfig) -> Self {
        Self { server }
    }

    pub fn server_ref(&self) -> &ServerConfig {
        &self.server
    }
}

impl std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
        }
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_toml_to_config::<AppConfigCrude, AppConfig>(file_name).unwrap_or_else(Self::default)
    }
}
