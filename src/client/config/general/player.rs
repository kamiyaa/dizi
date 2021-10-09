use std::path::PathBuf;

use serde_derive::Deserialize;

use crate::config::Flattenable;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawPlayerOption {
    #[serde(default)]
    pub shuffle: bool,
    #[serde(default = "default_true")]
    pub repeat: bool,
    #[serde(default)]
    pub next: bool,
    #[serde(default)]
    pub on_song_change: Option<String>,
}

impl Flattenable<PlayerOption> for RawPlayerOption {
    fn flatten(self) -> PlayerOption {
        PlayerOption {
            shuffle: self.shuffle,
            repeat: self.repeat,
            next: self.next,
            on_song_change: self.on_song_change,
        }
    }
}

impl std::default::Default for RawPlayerOption {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: true,
            next: true,
            on_song_change: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerOption {
    pub shuffle: bool,
    pub repeat: bool,
    pub next: bool,
    pub on_song_change: Option<String>,
}

impl std::default::Default for PlayerOption {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: true,
            next: true,
            on_song_change: None,
        }
    }
}
