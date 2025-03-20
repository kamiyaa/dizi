use rand::prelude::SliceRandom;
use rand::rng;

use dizi::song::DiziSongEntry;

use super::DiziPlaylist;
use crate::traits::{DiziPlaylistEntry, DiziPlaylistTrait};

impl DiziPlaylistTrait for DiziPlaylist {
    fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }
    fn len(&self) -> usize {
        self.contents.len()
    }
    fn push(&mut self, song: DiziSongEntry) {
        self.contents.push(song);
        let index = self.len() - 1;
        // add song to end of playlist order
        self.order.push(index);
    }
    fn remove(&mut self, index: usize) {
        self.contents.remove(index);
    }
    fn clear(&mut self) {
        self.contents.clear();
        self.order.clear();
        self.order_index = None;
    }
    fn swap(&mut self, index1: usize, index2: usize) {
        self.contents.swap(index1, index2);
        // if one of the songs is the one curently being played,
        // swap the playlist index as well
        if let Some(index) = self.order_index {
            if index == index1 {
                self.order_index = Some(index2);
            }
            if index == index2 {
                self.order_index = Some(index1);
            }
        }
    }
    fn is_end(&self) -> bool {
        match self.order_index {
            None => true,
            Some(i) => i + 1 >= self.len(),
        }
    }

    fn entry_ref(&self, index: usize) -> &DiziSongEntry {
        &self.contents[index]
    }

    fn current_entry(&self) -> Option<DiziPlaylistEntry> {
        let order_index = self.order_index?;
        let playlist_index = self.order[order_index];

        Some(DiziPlaylistEntry {
            entry_index: playlist_index,
            order_index,
            entry: self.entry_ref(playlist_index).clone(),
        })
    }

    fn next_song_peak(&self) -> Option<DiziPlaylistEntry> {
        let order_index = self.order_index?;
        let order_index = (order_index + 1) % self.len();

        let entry_index = self.order[order_index];

        Some(DiziPlaylistEntry {
            entry_index,
            order_index,
            entry: self.entry_ref(entry_index).clone(),
        })
    }
    fn previous_song_peak(&self) -> Option<DiziPlaylistEntry> {
        let order_index = self.order_index?;
        let order_index = (order_index + self.len() - 1) % self.len();

        let entry_index = self.order[order_index];

        Some(DiziPlaylistEntry {
            entry_index,
            order_index,
            entry: self.entry_ref(entry_index).clone(),
        })
    }

    fn shuffle(&mut self) {
        // the current song being played should be the
        // first value of the random order
        match self.current_entry() {
            Some(entry) => {
                let entry_index = entry.entry_index;
                let mut new_shuffle_order: Vec<usize> =
                    (0..self.len()).filter(|i| *i != entry_index).collect();
                new_shuffle_order.shuffle(&mut rng());
                new_shuffle_order.insert(0, entry_index);

                self.order = new_shuffle_order;
                self.order_index = Some(0);
            }
            None => {
                let mut new_shuffle_order: Vec<usize> = (0..self.len()).collect();
                new_shuffle_order.shuffle(&mut rng());
                self.order = new_shuffle_order;
            }
        }
    }

    fn unshuffle(&mut self) {
        // make sure unshuffle doesn't cause us to forget which song we were on
        if let Some(playlist_index) = self.order_index {
            let song_index = self.order[playlist_index];
            self.order_index = Some(song_index);
        }
        self.order = (0..self.len()).collect();
    }
}
