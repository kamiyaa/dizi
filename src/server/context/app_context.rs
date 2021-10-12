use std::sync::mpsc;

use dizi_lib::error::DiziResult;

use crate::config;
use crate::events::Events;

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
    pub events: Events,
    quit: QuitType,
    config: config::AppConfig,
    player_context: PlayerContext,
}

impl AppContext {
    pub fn new(config: config::AppConfig) -> Self {
        let events = Events::new();
        let event_tx2 = events.server_event_sender().clone();
        let player_context = PlayerContext::new(&config, event_tx2);
        Self {
            events,
            quit: QuitType::DoNot,
            config,
            player_context,
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
