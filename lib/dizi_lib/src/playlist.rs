use std::fs;
use std::io;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::song::Song;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Playlist {
    #[serde(rename = "list")]
    _list: Vec<Song>,
    index: usize,
}

impl Playlist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first_index_for_viewport(&self, viewport_height: usize) -> usize {
        match self.get_index() {
            Some(index) => index / viewport_height as usize * viewport_height as usize,
            None => 0,
        }
    }

    pub fn playlist(&self) -> &[Song] {
        self._list.as_slice()
    }

    pub fn append_song(&mut self, s: Song) {
        self._list.push(s);
    }

    pub fn remove_song(&mut self, index: usize) -> Song {
        let song = self.list_mut().remove(index);
        if self.list_ref().is_empty() {
            self.index = 0;
        } else if self.index >= self.list_ref().len() {
            self.index = self.list_ref().len() - 1;
        }
        song
    }

    pub fn get_index(&self) -> Option<usize> {
        if self.list_ref().is_empty() {
            None
        } else {
            Some(self.index)
        }
    }
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn len(&self) -> usize {
        self._list.len()
    }

    pub fn list_ref(&self) -> &Vec<Song> {
        &self._list
    }
    pub fn list_mut(&mut self) -> &mut Vec<Song> {
        &mut self._list
    }
}

impl std::default::Default for Playlist {
    fn default() -> Self {
        Self {
            _list: Vec::new(),
            index: 0,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HashPlaylist {
    #[serde(skip_serializing)]
    _set: HashSet<PathBuf>,
    #[serde(rename = "list")]
    _list: Vec<Song>,
    pub index: usize,
}

impl HashPlaylist {
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

    pub fn list_ref(&self) -> &Vec<Song> {
        &self._list
    }
    pub fn list_mut(&mut self) -> &mut Vec<Song> {
        &mut self._list
    }
}

impl std::default::Default for HashPlaylist {
    fn default() -> Self {
        Self {
            _set: HashSet::new(),
            _list: Vec::new(),
            index: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DirlistPlaylist {
    _list: Vec<PathBuf>,
    pub index: usize,
}

impl DirlistPlaylist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from(path: &Path) -> io::Result<Self> {
        let results: Vec<PathBuf> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .collect();
        Ok(Self {
            _list: results,
            index: 0,
        })
    }

    pub fn len(&self) -> usize {
        self._list.len()
    }

    pub fn list_ref(&self) -> &Vec<PathBuf> {
        &self._list
    }
    pub fn list_mut(&mut self) -> &mut Vec<PathBuf> {
        &mut self._list
    }
}

impl std::default::Default for DirlistPlaylist {
    fn default() -> Self {
        Self {
            _list: Vec::new(),
            index: 0,
        }
    }
}

