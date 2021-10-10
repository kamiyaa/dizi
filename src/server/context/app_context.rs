use std::sync::mpsc;

use dizi_lib::error::DiziResult;

use crate::config;

use super::PlayerContext;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuitType {
    DoNot,
    Normal,
    Force,
    ToCurrentDirectory,
}

#[derive(Debug)]
pub struct AppContext {
    quit: QuitType,
    config: config::AppConfig,
    player_context: PlayerContext,
}

impl AppContext {
    pub fn new(config: config::AppConfig) -> Self {
        Self {
            quit: QuitType::DoNot,
            config,
            player_context: PlayerContext::new(),
        }
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
