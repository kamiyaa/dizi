use crate::audio::player::Player;
use crate::config;
use crate::events::Events;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuitType {
    DoNot,
    Server,
}

#[derive(Debug)]
pub struct AppContext {
    pub events: Events,
    pub quit: QuitType,
    config: config::AppConfig,
    player: Player,
}

impl AppContext {
    pub fn new(config: config::AppConfig) -> Self {
        let events = Events::new();
        let event_tx2 = events.server_event_sender().clone();
        let player = Player::new(&config, event_tx2);
        Self {
            events,
            quit: QuitType::DoNot,
            config,
            player,
        }
    }

    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }

    pub fn player_ref(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
