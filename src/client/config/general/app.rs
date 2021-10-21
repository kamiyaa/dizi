use serde_derive::Deserialize;

use crate::config::option::{DisplayOption, SortOption};
use crate::config::{parse_to_config_file, TomlConfigFile};

use super::client::{ClientConfig, ClientConfigCrude};

const fn default_true() -> bool {
    true
}
const fn default_scroll_offset() -> usize {
    6
}
const fn default_max_preview_size() -> u64 {
    2 * 1024 * 1024 // 2 MB
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfigCrude {
    #[serde(default)]
    pub client: ClientConfigCrude,
}

impl From<AppConfigCrude> for AppConfig {
    fn from(crude: AppConfigCrude) -> Self {
        Self {
            _client: ClientConfig::from(crude.client),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    _client: ClientConfig,
}

impl AppConfig {
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

impl std::default::Default for AppConfig {
    fn default() -> Self {
        Self {
            _client: ClientConfig::default(),
        }
    }
}

impl TomlConfigFile for AppConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<AppConfigCrude, AppConfig>(file_name).unwrap_or_else(Self::default)
    }
}
