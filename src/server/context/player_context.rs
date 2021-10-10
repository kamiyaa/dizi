use dizi_lib::error::DiziResult;
use std::sync::{Arc, Mutex};

use crate::audio::Player;

#[derive(Debug)]
pub struct PlayerContext {
    _player: Arc<Mutex<Player>>,
}

impl PlayerContext {
    pub fn new() -> Self {
        let player = Player::new();
        Self {
            _player: Arc::new(Mutex::new(player)),
        }
    }

    pub fn player_clone(&self) -> Arc<Mutex<Player>> {
        self._player.clone()
    }

    pub fn player_ref(&self) -> &Arc<Mutex<Player>> {
        &self._player
    }

    pub fn player_mut(&mut self) -> &mut Arc<Mutex<Player>> {
        &mut self._player
    }
}
