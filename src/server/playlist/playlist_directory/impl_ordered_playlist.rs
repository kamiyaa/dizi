use dizi_lib::song::Song;

use super::PlaylistDirectory;
use crate::traits::{OrderedPlaylist, OrderedPlaylistEntry};

impl OrderedPlaylist for PlaylistDirectory {
    fn is_empty(&self) -> bool {
        self.playlist_content_ref().is_empty()
    }
    fn len(&self) -> usize {
        self.playlist_content_ref().len()
    }
    fn push(&mut self, song: Song) {
        self.playlist_content_mut().push(song);
        let index = self.len() - 1;
        self.playlist_order_mut().push(index);
    }
    fn remove(&mut self, index: usize) {
        self.playlist_content_mut().remove(index);
    }
    fn clear(&mut self) {
        self.playlist_content_mut().clear();
        self.playlist_order_mut().clear();
        self._playlist_index = None;
    }
    fn iter(&self) -> std::slice::Iter<'_, Song> {
        self.playlist_content_ref().iter()
    }
    fn swap(&mut self, index1: usize, index2: usize) {
        self.playlist_content_mut().swap(index1, index2);
        if let Some(index) = self.get_playlist_index() {
            if index == index1 {
                self.set_playlist_index(Some(index2));
            }
            if index == index2 {
                self.set_playlist_index(Some(index1));
            }
        }
    }
    fn is_end(&self) -> bool {
        match self.get_playlist_index() {
            None => true,
            Some(i) => i + 1 >= self.len(),
        }
    }

    fn entry_ref(&self, index: usize) -> &Song {
        &self.playlist_content_ref()[index]
    }
    fn entry_mut(&mut self, index: usize) -> &mut Song {
        &mut self.playlist_content_mut()[index]
    }

    fn current_entry_details(&self) -> Option<OrderedPlaylistEntry> {
        let playlist_index = self.get_playlist_index()?;
        let song_index = self.playlist_order_ref()[playlist_index];

        Some(OrderedPlaylistEntry {
            song_index,
            playlist_index,
            entry: self.entry_ref(song_index).clone(),
        })
    }

    fn next_song_peak(&self) -> Option<OrderedPlaylistEntry> {
        let playlist_index = self.get_playlist_index()?;
        let playlist_index = (playlist_index + 1) % self.len();

        let song_index = self.playlist_order_ref()[playlist_index];

        Some(OrderedPlaylistEntry {
            song_index,
            playlist_index,
            entry: self.entry_ref(song_index).clone(),
        })
    }
    fn previous_song_peak(&self) -> Option<OrderedPlaylistEntry> {
        let playlist_index = self.get_playlist_index()?;
        let playlist_index = (playlist_index + self.len() - 1) % self.len();

        let song_index = self.playlist_order_ref()[playlist_index];

        Some(OrderedPlaylistEntry {
            song_index,
            playlist_index,
            entry: self.entry_ref(song_index).clone(),
        })
    }

    fn next_song(&mut self) -> Option<OrderedPlaylistEntry> {
        let playlist_index = self.get_playlist_index()?;
        let playlist_index = (playlist_index + 1) % self.len();
        self.set_playlist_index(Some(playlist_index));

        let song_index = self.playlist_order_ref()[playlist_index];

        Some(OrderedPlaylistEntry {
            song_index,
            playlist_index,
            entry: self.entry_ref(song_index).clone(),
        })
    }
    fn previous_song(&mut self) -> Option<OrderedPlaylistEntry> {
        let playlist_index = self.get_playlist_index()?;
        let playlist_index = (playlist_index + self.len() - 1) % self.len();
        self.set_playlist_index(Some(playlist_index));

        let song_index = self.playlist_order_ref()[playlist_index];

        Some(OrderedPlaylistEntry {
            song_index,
            playlist_index,
            entry: self.entry_ref(song_index).clone(),
        })
    }
}
