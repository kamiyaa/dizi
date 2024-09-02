use serde::Deserialize;

use crate::config::option::{DisplayOption, SortOption};
use crate::config::{parse_toml_to_config, TomlConfigFile};

use super::client::{ClientConfig, ClientConfigRaw};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigRaw {
    #[serde(default)]
    pub client: ClientConfigRaw,
}

impl From<AppConfigRaw> for AppConfig {
    fn from(raw: AppConfigRaw) -> Self {
        Self {
            _client: ClientConfig::from(raw.client),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct AppConfig {
    _client: ClientConfig,
}

impl AppConfig {
    #[allow(dead_code)]
    pub fn new(client: ClientConfig) -> Self {
        Self { _client: client }
    }

    pub fn client_ref(&self) -> &ClientConfig {
        &self._client
    }

    pub fn client_mut(&mut self) -> &mut ClientConfig {
        &mut self._client
    }

    pub fn display_options_ref(&self) -> &DisplayOption {
        &self.client_ref().display_options
    }
    pub fn display_options_mut(&mut self) -> &mut DisplayOption {
        &mut self.client_mut().display_options
    }

    pub fn sort_options_ref(&self) -> &SortOption {
        self.display_options_ref().sort_options_ref()
    }
    pub fn sort_options_mut(&mut self) -> &mut SortOption {
        self.display_options_mut().sort_options_mut()
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
