use serde::Deserialize;

use crate::config::{parse_toml_to_config, TomlConfigFile};

use super::{ServerConfig, ServerConfigRaw};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigRaw {
    #[serde(default)]
    pub server: ServerConfigRaw,
}

#[derive(Clone, Debug, Default)]
pub struct AppConfig {
    server: ServerConfig,
}

impl From<AppConfigRaw> for AppConfig {
    fn from(crude: AppConfigRaw) -> Self {
        Self {
            server: ServerConfig::from(crude.server),
        }
    }
}

impl AppConfig {
    pub fn server_ref(&self) -> &ServerConfig {
        &self.server
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        match parse_toml_to_config::<AppConfigRaw, AppConfig>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse server config: {}", e);
                Self::default()
            }
        }
    }
}
