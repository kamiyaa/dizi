use serde_derive::Deserialize;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayerOptionCrude {
    #[serde(default)]
    pub shuffle: bool,
    #[serde(default = "default_true")]
    pub repeat: bool,
    #[serde(default = "default_true")]
    pub next: bool,
}

impl std::default::Default for PlayerOptionCrude {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: true,
            next: true,
        }
    }
}

impl From<PlayerOptionCrude> for PlayerOption {
    fn from(crude: PlayerOptionCrude) -> Self {
        Self {
            shuffle: crude.shuffle,
            repeat: crude.repeat,
            next: crude.next,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerOption {
    pub shuffle: bool,
    pub repeat: bool,
    pub next: bool,
}

impl std::default::Default for PlayerOption {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: true,
            next: true,
        }
    }
}
