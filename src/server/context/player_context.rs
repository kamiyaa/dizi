use crate::audio::Player;
use dizi_commands::error::DiziResult;

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
