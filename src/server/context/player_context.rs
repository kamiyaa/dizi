use std::sync::{Arc, Mutex};

use crate::audio::Player;
use crate::config;
use crate::events::ServerEventSender;

#[derive(Debug)]
pub struct PlayerContext {
    _player: Player,
}

impl PlayerContext {
    pub fn new(config_t: &config::AppConfig, event: ServerEventSender) -> Self {
        let player = Player::new(config_t, event);
        Self { _player: player }
    }

    pub fn player_ref(&self) -> &Player {
        &self._player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self._player
    }
}
