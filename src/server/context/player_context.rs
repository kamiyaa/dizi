use dizi_lib::error::DiziResult;

use crate::audio::Player;

#[derive(Debug)]
pub struct PlayerContext {
    player: Player,
}

impl PlayerContext {
    pub fn new() -> DiziResult<Self> {
        let player = Player::new();
        Ok(Self { player })
    }

    pub fn player_ref(&self) -> &Player {
        &self.player
    }
    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
