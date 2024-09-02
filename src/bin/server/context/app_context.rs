use crate::audio::symphonia::player::SymphoniaPlayer;
use crate::config;
use crate::events::Events;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitType {
    DoNot,
    Server,
}

#[derive(Debug)]
pub struct AppContext {
    pub config: config::AppConfig,
    pub events: Events,
    pub quit: QuitType,
    pub player: SymphoniaPlayer,
}

impl AppContext {
    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }
}
