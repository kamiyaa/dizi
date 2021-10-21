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
