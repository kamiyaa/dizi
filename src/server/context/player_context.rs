use rodio::{Decoder, OutputStream, OutputStreamHandle};
use rodio::source;

use dizi_commands::error::DiziResult;
use crate::audio::Player;

#[derive(Clone)]
pub struct PlayerContext {
    player: Player,
}

impl PlayerContext {
    pub fn new() -> DiziResult<Self> {
        let player = Player::new();
        Ok(Self {
            player,
        })
    }

    pub fn player_ref(&self) -> &Player {
        &self.player
    }
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
