use serde::{Deserialize, Serialize};

use dizi::song::DiziSongEntry;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiziPlaylistEntry {
    pub entry_index: usize,
    pub order_index: usize,
    pub entry: DiziSongEntry,
}

pub trait DiziPlaylistTrait {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&mut self, song: DiziSongEntry);
    fn remove(&mut self, index: usize);
    fn clear(&mut self);
    fn swap(&mut self, index1: usize, index2: usize);

    fn is_end(&self) -> bool;

    fn entry_ref(&self, index: usize) -> &DiziSongEntry;

    fn current_entry(&self) -> Option<DiziPlaylistEntry>;

    fn next_song_peak(&self) -> Option<DiziPlaylistEntry>;
    fn previous_song_peak(&self) -> Option<DiziPlaylistEntry>;

    fn shuffle(&mut self);
    fn unshuffle(&mut self);
}
