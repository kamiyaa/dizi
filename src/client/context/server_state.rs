use std::time;

use dizi_lib::player::PlayerState;
use dizi_lib::playlist::Playlist;
use dizi_lib::song::Song;

#[derive(Clone, Debug)]
pub struct ServerState {
    player: PlayerState,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            player: PlayerState::new(),
        }
    }

    pub fn set_player(&mut self, player: PlayerState) {
        self.player = player;
    }

    pub fn player_ref(&self) -> &PlayerState {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut PlayerState {
        &mut self.player
    }
}
