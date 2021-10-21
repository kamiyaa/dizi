use serde_derive::Deserialize;

use crate::config::option::PlayerOption;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayerOptionCrude {
    #[serde(default)]
    pub shuffle: bool,
    #[serde(default = "default_true")]
    pub repeat: bool,
    #[serde(default)]
    pub next: bool,
    #[serde(default)]
    pub on_song_change: Option<String>,
}

impl std::default::Default for PlayerOptionCrude {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: true,
            next: true,
            on_song_change: None,
        }
    }
}

impl From<PlayerOptionCrude> for PlayerOption {
    fn from(crude: PlayerOptionCrude) -> Self {
        Self {
            shuffle: crude.shuffle,
            repeat: crude.repeat,
            next: crude.next,
            on_song_change: crude.on_song_change,
        }
    }
}
