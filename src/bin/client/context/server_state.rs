use dizi::player::PlayerState;

#[derive(Clone, Debug)]
pub struct ServerState {
    pub player: PlayerState,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            player: PlayerState::new(),
        }
    }
}
