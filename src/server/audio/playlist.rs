use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use rand::seq::SliceRandom;
use rand::thread_rng;

use dizi_lib::playlist::{FilePlaylist, PlaylistStatus};
use dizi_lib::song::Song;

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
    _shuffle: bool,
}

impl PlayerFilePlaylist {
    pub fn new(songs: Vec<Song>) -> Self {
        let songs_count = songs.len();
        Self {
            _songs: songs,
            _order: (0..songs_count).collect(),
            _order_index: None,
            _shuffle: false,
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
                        if let Ok(song) = Song::new(&p) {
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

    pub fn is_empty(&self) -> bool {
        self._songs.is_empty()
    }
    pub fn len(&self) -> usize {
        self._songs.len()
    }
    pub fn push(&mut self, song: Song) {
        self._songs.push(song);
    }
    pub fn remove(&mut self, index: usize) {
        self._songs.remove(index);
    }
    pub fn clear(&mut self) {
        self._songs.clear();
        self._order = vec![];
        self._order_index = None;
    }

    pub fn clone_file_playlist(&self) -> FilePlaylist {
        FilePlaylist {
            list: self._songs.clone(),
            cursor_index: None,
            playing_index: self.get_song_index(),
        }
    }

    pub fn set_shuffle(&mut self, shuffle: bool) {
        self._shuffle = shuffle;
    }

    pub fn songs_ref(&self) -> &Vec<Song> {
        &self._songs
    }
    pub fn songs_mut(&mut self) -> &mut Vec<Song> {
        &mut self._songs
    }

    pub fn playlist_order_ref(&self) -> &Vec<usize> {
        &self._order
    }
    pub fn playlist_order_mut(&mut self) -> &mut Vec<usize> {
        &mut self._order
    }

    pub fn get_song_index(&self) -> Option<usize> {
        self._order_index.clone().and_then(|s| Some(self._order[s]))
    }
    pub fn set_song_index(&mut self, index: usize) {
        if self.len() <= index {
            return;
        }

        if self._shuffle {
            let mut random_order: Vec<usize> = (0..self.len()).collect();
            // we want the song to be first
            random_order.remove(index);
            random_order.shuffle(&mut thread_rng());

            let mut order = vec![index];
            order.extend_from_slice(&random_order);
            self._order = order;
            self.set_order_index(Some(0));
        } else {
            let order = (0..self.len()).collect();
            self._order = order;
            self.set_order_index(Some(index));
        }
    }
    pub fn get_order_index(&self) -> Option<usize> {
        self._order_index.clone()
    }
    pub fn set_order_index(&mut self, index: Option<usize>) {
        self._order_index = index;
    }

    pub fn increment_order_index(&mut self) -> Option<usize> {
        let order_index = self.get_order_index()?;
        let new_order_index = if order_index + 1 < self.len() {
            order_index + 1
        } else {
            0
        };
        self.set_order_index(Some(new_order_index));
        Some(new_order_index)
    }
    pub fn decrement_order_index(&mut self) -> Option<usize> {
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

#[derive(Clone, Debug)]
pub struct PlayerDirectoryPlaylist {
    _songs: Vec<PathBuf>,
    _order: Vec<usize>,
    _order_index: Option<usize>,
    _shuffle: bool,
}

impl PlayerDirectoryPlaylist {
    pub fn new(songs: Vec<PathBuf>) -> Self {
        let songs_count = songs.len();
        Self {
            _songs: songs,
            _order: (0..songs_count).collect(),
            _order_index: None,
            _shuffle: false,
        }
    }

    pub fn from_path(path: &Path) -> io::Result<Self> {
        let results: Vec<PathBuf> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .collect();

        let len = results.len();
        Ok(Self {
            _songs: results,
            _order: (0..len).collect(),
            _order_index: None,
            _shuffle: false,
        })
    }

    pub fn is_empty(&self) -> bool {
        self._songs.is_empty()
    }
    pub fn len(&self) -> usize {
        self._songs.len()
    }
    pub fn push(&mut self, song: PathBuf) {
        self._songs.push(song);
    }
    pub fn remove(&mut self, index: usize) {
        self._songs.remove(index);
    }
    pub fn clear(&mut self) {
        self._songs.clear();
        self._order = vec![];
        self._order_index = None;
    }

    pub fn set_shuffle(&mut self, shuffle: bool) {
        self._shuffle = shuffle;
    }

    pub fn songs_ref(&self) -> &Vec<PathBuf> {
        &self._songs
    }
    pub fn songs_mut(&mut self) -> &mut Vec<PathBuf> {
        &mut self._songs
    }

    pub fn playlist_order_ref(&self) -> &Vec<usize> {
        &self._order
    }
    pub fn playlist_order_mut(&mut self) -> &mut Vec<usize> {
        &mut self._order
    }

    pub fn get_song_index(&self) -> Option<usize> {
        self._order_index.clone().and_then(|s| Some(self._order[s]))
    }
    pub fn set_song_index(&mut self, index: usize) {
        if self.len() <= index {
            return;
        }

        if self._shuffle {
            let mut random_order: Vec<usize> = (0..self.len()).collect();
            // we want the song to be first
            random_order.remove(index);
            random_order.shuffle(&mut thread_rng());

            let mut order = vec![index];
            order.extend_from_slice(&random_order);
            self._order = order;
            self.set_order_index(Some(0));
        } else {
            let order = (0..self.len()).collect();
            self._order = order;
            self.set_order_index(Some(index));
        }
    }
    pub fn get_order_index(&self) -> Option<usize> {
        self._order_index.clone()
    }
    pub fn set_order_index(&mut self, index: Option<usize>) {
        self._order_index = index;
    }

    pub fn increment_order_index(&mut self) -> Option<usize> {
        let order_index = self.get_order_index()?;
        let new_order_index = if order_index + 1 < self.len() {
            order_index + 1
        } else {
            0
        };
        self.set_order_index(Some(new_order_index));
        Some(new_order_index)
    }
    pub fn decrement_order_index(&mut self) -> Option<usize> {
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
