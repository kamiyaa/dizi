use serde::Deserialize;

use crate::config::{TomlConfigFile, parse_toml_to_config};

use super::client::{ClientConfig, ClientConfigRaw};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigRaw {
    #[serde(default)]
    pub client: ClientConfigRaw,
}

impl From<AppConfigRaw> for AppConfig {
    fn from(raw: AppConfigRaw) -> Self {
        Self {
            client: ClientConfig::from(raw.client),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AppConfig {
    pub client: ClientConfig,
}

impl AppConfig {
    #[allow(dead_code)]
    pub fn new(client: ClientConfig) -> Self {
        Self { client }
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        match parse_toml_to_config::<AppConfigRaw, AppConfig>(file_name) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to parse client config: {}", e);
                Self::default()
            }
        }
    }
}
