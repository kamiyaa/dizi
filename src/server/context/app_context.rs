#[cfg(feature = "rodio-backend")]
use crate::audio::rodio::player::RodioPlayer;
#[cfg(feature = "symphonia-backend")]
use crate::audio::symphonia::player::SymphoniaPlayer;
use crate::config;
use crate::events::Events;
use crate::traits::AudioPlayer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitType {
    DoNot,
    Server,
}

#[derive(Debug)]
pub struct AppContext {
    pub events: Events,
    pub quit: QuitType,
    config: config::AppConfig,
    #[cfg(feature = "rodio-backend")]
    player: RodioPlayer,
    #[cfg(feature = "symphonia-backend")]
    player: SymphoniaPlayer,
}

impl AppContext {
    pub fn new(config: config::AppConfig) -> Self {
        let events = Events::new();
        let event_tx2 = events.server_event_sender().clone();

        #[cfg(feature = "rodio-backend")]
        let player = RodioPlayer::new(&config, event_tx2);
        #[cfg(feature = "symphonia-backend")]
        let player = SymphoniaPlayer::new(&config, event_tx2);
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

    pub fn player_ref(&self) -> &dyn AudioPlayer {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut dyn AudioPlayer {
        &mut self.player
    }
}
