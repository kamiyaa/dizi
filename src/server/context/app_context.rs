use dizi_commands::error::DiziResult;

use crate::config;

use super::PlayerContext;

#[derive(Debug)]
pub struct AppContext {
    config: config::AppConfig,
    player_context: PlayerContext,
}

impl AppContext {
    pub fn new(config: config::AppConfig) -> DiziResult<Self> {
        Ok(Self {
            config,
            player_context: PlayerContext::new()?,
        })
    }

    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }

    pub fn player_context_ref(&self) -> &PlayerContext {
        &self.player_context
    }
    pub fn player_context_mut(&mut self) -> &mut PlayerContext {
        &mut self.player_context
    }
}
