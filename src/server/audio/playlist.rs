use std::fs;
use std::io;
use std::path::Path;

use rand::seq::SliceRandom;
use rand::thread_rng;

use dizi_lib::playlist::{FilePlaylist, PlaylistStatus};
use dizi_lib::song::Song;

pub trait DiziPlaylist {
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn push(&mut self, song: Song);
    fn remove(&mut self, index: usize);
    fn clear(&mut self);
    fn iter(&self) -> std::slice::Iter<'_, Song>;
    fn swap(&mut self, index1: usize, index2: usize);

    fn get_song(&self, index: usize) -> &Song;

    fn playlist_order_ref(&self) -> &Vec<usize>;
    fn playlist_order_mut(&mut self) -> &mut Vec<usize>;

    fn get_song_index(&self) -> Option<usize>;
    fn set_song_index(&mut self, index: usize);

    fn shuffle_enabled(&self) -> bool;
    fn set_shuffle(&mut self, shuffle: bool);

    fn get_order_index(&self) -> Option<usize>;
    fn set_order_index(&mut self, index: Option<usize>);

    fn increment_order_index(&mut self) -> Option<usize>;
    fn decrement_order_index(&mut self) -> Option<usize>;

    fn reached_end(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct PlayerPlaylist {
    pub file_playlist: PlayerFilePlaylist,
    pub directory_playlist: PlayerDirectoryPlaylist,
    pub status: PlaylistStatus,
}

impl PlayerPlaylist {
    pub fn get_status(&self) -> PlaylistStatus {
        self.status
    }
    pub fn set_status(&mut self, status: PlaylistStatus) {
        self.status = status;
    }

    pub fn file_playlist_ref(&self) -> &PlayerFilePlaylist {
        &self.file_playlist
    }
    pub fn file_playlist_mut(&mut self) -> &mut PlayerFilePlaylist {
        &mut self.file_playlist
    }

    pub fn directory_playlist_ref(&self) -> &PlayerDirectoryPlaylist {
        &self.directory_playlist
    }
    pub fn directory_playlist_mut(&mut self) -> &mut PlayerDirectoryPlaylist {
        &mut self.directory_playlist
    }
}

impl std::default::Default for PlayerPlaylist {
    fn default() -> Self {
        Self {
            file_playlist: PlayerFilePlaylist::new(Vec::new()),
            directory_playlist: PlayerDirectoryPlaylist::new(Vec::new()),
            status: PlaylistStatus::PlaylistFile,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerFilePlaylist {
    _songs: Vec<Song>,
    _order: Vec<usize>,
    _order_index: Option<usize>,
    shuffle: bool,
}

impl PlayerFilePlaylist {
    pub fn new(songs: Vec<Song>) -> Self {
        let songs_count = songs.len();
        Self {
            _songs: songs,
            _order: (0..songs_count).collect(),
            _order_index: None,
            shuffle: false,
        }
    }

    pub fn from_file(cwd: &Path, path: &Path) -> io::Result<PlayerFilePlaylist> {
        let mut reader = m3u::Reader::open(path)?;
        let read_playlist: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
        let mut songs = Vec::new();
        for entry in &read_playlist {
            match entry {
                m3u::Entry::Path(p) => {
                    if p.is_absolute() {
                        if let Ok(song) = Song::new(p) {
                            songs.push(song);
                        }
                    } else {
                        let mut new_path = cwd.to_path_buf();
                        new_path.push(p);
                        if let Ok(song) = Song::new(&new_path) {
                            songs.push(song);
                        }
                    }
                }
                _ => {}
            }
        }
        let playlist = PlayerFilePlaylist::new(songs);
        Ok(playlist)
    }

    pub fn clone_file_playlist(&self) -> FilePlaylist {
        FilePlaylist {
            list: self._songs.clone(),
            cursor_index: None,
            playing_index: self.get_song_index(),
        }
    }

    fn on_spot_shuffle(&mut self) {
        let mut random_order: Vec<usize> = (0..self.len()).collect();

        // we want the current song's index to not change
        if let Some(index) = self.get_order_index() {
            random_order.remove(index);
            random_order.shuffle(&mut thread_rng());
            random_order.insert(0, index);
            self.set_order_index(Some(0));
        } else {
            random_order.shuffle(&mut thread_rng());
        }
        self._order = random_order;
    }

    fn on_spot_unshuffle(&mut self) {
        self._order = (0..self.len()).collect();
    }
}

impl DiziPlaylist for PlayerFilePlaylist {
    fn is_empty(&self) -> bool {
        self._songs.is_empty()
    }
    fn len(&self) -> usize {
        self._songs.len()
    }
    fn push(&mut self, song: Song) {
        self._songs.push(song);
        self._order.push(self.len() - 1);
    }
    fn remove(&mut self, index: usize) {
        self._songs.remove(index);
    }
    fn clear(&mut self) {
        self._songs.clear();
        self._order = Vec::new();
        self._order_index = None;
    }
    fn iter(&self) -> std::slice::Iter<'_, Song> {
        self._songs.iter()
    }
    fn swap(&mut self, index1: usize, index2: usize) {
        self._songs.swap(index1, index2);
    }
    fn reached_end(&self) -> bool {
        match self.get_order_index() {
            None => true,
            Some(i) => return i + 1 >= self.len(),
        }
    }

    fn get_song(&self, index: usize) -> &Song {
        &self._songs[index]
    }

    fn playlist_order_ref(&self) -> &Vec<usize> {
        &self._order
    }
    fn playlist_order_mut(&mut self) -> &mut Vec<usize> {
        &mut self._order
    }

    fn get_song_index(&self) -> Option<usize> {
        self._order_index.map(|s| self._order[s])
    }
    fn set_song_index(&mut self, index: usize) {
        if self.len() <= index {
            return;
        }
        self.set_order_index(Some(index));
    }

    fn shuffle_enabled(&self) -> bool {
        self.shuffle
    }
    fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;

        if self.is_empty() {
            return;
        }

        if self.shuffle_enabled() {
            self.on_spot_shuffle();
        } else {
            self.on_spot_unshuffle();
        }
    }

    fn get_order_index(&self) -> Option<usize> {
        self._order_index
    }
    fn set_order_index(&mut self, index: Option<usize>) {
        self._order_index = index;
    }

    fn increment_order_index(&mut self) -> Option<usize> {
        let order_index = self.get_order_index()?;
        let new_order_index = if order_index + 1 < self.len() {
            order_index + 1
        } else {
            0
        };
        self.set_order_index(Some(new_order_index));
        Some(new_order_index)
    }
    fn decrement_order_index(&mut self) -> Option<usize> {
        let order_index = self.get_order_index()?;
        let new_order_index = if order_index > 0 {
            order_index - 1
        } else {
            self.len() - 1
        };
        self.set_order_index(Some(new_order_index));
        Some(new_order_index)
    }
}

#[derive(Clone, Debug)]
pub struct PlayerDirectoryPlaylist {
    _songs: Vec<Song>,
    _order: Vec<usize>,
    _order_index: Option<usize>,
    shuffle: bool,
}

impl PlayerDirectoryPlaylist {
    pub fn new(songs: Vec<Song>) -> Self {
        let songs_count = songs.len();
        Self {
            _songs: songs,
            _order: (0..songs_count).collect(),
            _order_index: None,
            shuffle: false,
        }
    }

    pub fn from_path(path: &Path) -> io::Result<Self> {
        // only process regular files
        // if we can't read it, then don't play it
        let songs: Vec<Song> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .filter_map(|path| Song::new(&path).ok())
            .collect();

        let len = songs.len();
        Ok(Self {
            _songs: songs,
            _order: (0..len).collect(),
            _order_index: None,
            shuffle: false,
        })
    }

    fn on_spot_shuffle(&mut self) {
        let mut random_order: Vec<usize> = (0..self.len()).collect();

        // we want the current song's index to not change
        if let Some(index) = self.get_order_index() {
            random_order.remove(index);
            random_order.shuffle(&mut thread_rng());
            random_order.insert(0, index);
            self.set_order_index(Some(0));
        } else {
            random_order.shuffle(&mut thread_rng());
        }
        self._order = random_order;
    }

    fn on_spot_unshuffle(&mut self) {
        self._order = (0..self.len()).collect();
    }
}

impl DiziPlaylist for PlayerDirectoryPlaylist {
    fn is_empty(&self) -> bool {
        self._songs.is_empty()
    }
    fn len(&self) -> usize {
        self._songs.len()
    }
    fn iter(&self) -> std::slice::Iter<'_, Song> {
        self._songs.iter()
    }
    fn swap(&mut self, index1: usize, index2: usize) {
        self._songs.swap(index1, index2);
    }
    fn push(&mut self, song: Song) {
        self._songs.push(song);
        self._order.push(self.len() - 1);
    }
    fn remove(&mut self, index: usize) {
        self._songs.remove(index);
    }
    fn clear(&mut self) {
        self._songs.clear();
        self._order = Vec::new();
        self._order_index = None;
    }

    fn reached_end(&self) -> bool {
        match self.get_order_index() {
            None => true,
            Some(i) => return i + 1 >= self.len(),
        }
    }

    fn playlist_order_ref(&self) -> &Vec<usize> {
        &self._order
    }
    fn playlist_order_mut(&mut self) -> &mut Vec<usize> {
        &mut self._order
    }

    fn get_song(&self, index: usize) -> &Song {
        &self._songs[index]
    }

    fn get_song_index(&self) -> Option<usize> {
        self._order_index.map(|s| self._order[s])
    }
    fn set_song_index(&mut self, index: usize) {
        if self.len() <= index {
            return;
        }
        self._order = (0..self.len()).collect();
        self.set_order_index(Some(index));
    }

    fn shuffle_enabled(&self) -> bool {
        self.shuffle
    }
    fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;

        if self.is_empty() {
            return;
        }

        if self.shuffle_enabled() {
            self.on_spot_shuffle();
        } else {
            self.on_spot_unshuffle();
        }
    }

    fn get_order_index(&self) -> Option<usize> {
        self._order_index.clone()
    }
    fn set_order_index(&mut self, index: Option<usize>) {
        self._order_index = index;
    }

    fn increment_order_index(&mut self) -> Option<usize> {
        let order_index = self.get_order_index()?;
        let new_order_index = if order_index + 1 < self.len() {
            order_index + 1
        } else {
            0
        };
        self.set_order_index(Some(new_order_index));
        Some(new_order_index)
    }
    fn decrement_order_index(&mut self) -> Option<usize> {
        let order_index = self.get_order_index()?;
        let new_order_index = if order_index > 1 {
            order_index - 1
        } else {
            self.len() - 1
        };
        self.set_order_index(Some(new_order_index));
        Some(new_order_index)
    }
}
