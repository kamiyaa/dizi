use rand::prelude::SliceRandom;
use rand::thread_rng;

use super::PlaylistFile;
use crate::traits::{OrderedPlaylist, ShufflePlaylist};

impl ShufflePlaylist for PlaylistFile {
    fn shuffle(&mut self) {
        let mut random_order: Vec<usize> = (0..self.len()).collect();

        // the current song being played should be the
        // first value of the random order
        match self.current_entry_details() {
            Some(entry) => {
                random_order.remove(entry.song_index);
                random_order.shuffle(&mut thread_rng());
                random_order.insert(0, entry.song_index);
                self.set_playlist_index(Some(0));
            }
            None => {
                random_order.shuffle(&mut thread_rng());
            }
        }
        self._playlist_order = random_order;
    }

    fn unshuffle(&mut self) {
        // make sure unshuffle doesn't cause us to forget which song we were on
        if let Some(playlist_index) = self.get_playlist_index() {
            let song_index = self.playlist_order_ref()[playlist_index];
            self.set_playlist_index(Some(song_index));
        }
        self._playlist_order = (0..self.len()).collect();
    }
}
