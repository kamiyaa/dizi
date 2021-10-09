use std::sync::mpsc;

use dizi_lib::error::DiziResult;

use crate::config;
use crate::events::AppEvent;

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
    pub event_tx: mpsc::Sender<AppEvent>,
    config: config::AppConfig,
    player_context: PlayerContext,
}

impl AppContext {
    pub fn new(config: config::AppConfig, event_tx: mpsc::Sender<AppEvent>) -> DiziResult<Self> {
        Ok(Self {
            event_tx,
            config,
            player_context: PlayerContext::new()?,
        })
    }

    pub fn event_req(&self) -> &mpsc::Sender<AppEvent> {
        &self.event_tx
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
