use std::collections::HashSet;
use std::path::PathBuf;

use crate::song::Song;

#[derive(Clone, Debug)]
pub struct Playlist {
    _set: HashSet<PathBuf>,
    _list: Vec<Song>,
    pub index: usize,
}

impl Playlist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn playlist(&self) -> &[Song] {
        self._list.as_slice()
    }

    pub fn append_song(&mut self, s: Song) {
        self._set.insert(s.file_path().to_path_buf());
        self._list.push(s);
    }

    pub fn remove_song(&mut self, index: usize) -> Song {
        let song = self._list.remove(index);
        self._set.remove(&song.file_path().to_path_buf());
        song
    }

    pub fn len(&self) -> usize {
        self._list.len()
    }

    pub fn contains(&self, s: &PathBuf) -> bool {
        self._set.contains(s)
    }
}

impl std::default::Default for Playlist {
    fn default() -> Self {
        Self {
            _set: HashSet::new(),
            _list: Vec::new(),
            index: 0,
        }
    }
}
