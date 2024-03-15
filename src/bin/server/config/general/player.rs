use serde_derive::Deserialize;

const fn default_true() -> bool {
    true
}

const fn default_volume() -> usize {
    50
}

#[derive(Clone, Debug, Deserialize)]
pub struct PlayerOptionRaw {
    #[serde(default)]
    pub shuffle: bool,
    #[serde(default = "default_true")]
    pub repeat: bool,
    #[serde(default = "default_true")]
    pub next: bool,
    #[serde(default = "default_volume")]
    pub volume: usize,
}

impl std::default::Default for PlayerOptionRaw {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: true,
            next: true,
            volume: default_volume(),
        }
    }
}

impl From<PlayerOptionRaw> for PlayerOption {
    fn from(crude: PlayerOptionRaw) -> Self {
        Self {
            shuffle: crude.shuffle,
            repeat: crude.repeat,
            next: crude.next,
            volume: crude.volume,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerOption {
    pub shuffle: bool,
    pub repeat: bool,
    pub next: bool,
    pub volume: usize,
}

impl std::default::Default for PlayerOption {
    fn default() -> Self {
        Self {
            shuffle: false,
            repeat: true,
            next: true,
            volume: default_volume(),
        }
    }
}
