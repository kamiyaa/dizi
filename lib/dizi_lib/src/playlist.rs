use std::path::{Path, PathBuf};

use serde_derive::{Deserialize, Serialize};

use crate::song::Song;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlaylistStatus {
    DirectoryListing,
    PlaylistFile,
}

impl ToString for PlaylistStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::DirectoryListing => "directory".to_string(),
            Self::PlaylistFile => "file".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilePlaylist {
    #[serde(rename = "list")]
    pub list: Vec<Song>,
    pub cursor_index: Option<usize>,
    pub playing_index: Option<usize>,
}

impl FilePlaylist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn first_index_for_viewport(&self, viewport_height: usize) -> usize {
        match self.get_cursor_index() {
            Some(index) => index / viewport_height as usize * viewport_height as usize,
            None => 0,
        }
    }

    pub fn playlist(&self) -> &[Song] {
        self.list.as_slice()
    }

    pub fn clear(&mut self) {
        self.list_mut().clear();
        self.cursor_index = None;
        self.playing_index = None;
    }

    pub fn append_song(&mut self, s: Song) {
        self.list_mut().push(s);
    }

    pub fn remove_song(&mut self, index: usize) -> Song {
        let song = self.list_mut().remove(index);
        if self.list_ref().is_empty() {
            self.cursor_index = None;
        } else if let Some(index) = self.get_cursor_index() {
            if index >= self.list_ref().len() {
                self.set_cursor_index(Some(self.list_ref().len() - 1));
            }
        }
        song
    }

    pub fn get_cursor_index(&self) -> Option<usize> {
        self.cursor_index
    }
    pub fn set_cursor_index(&mut self, index: Option<usize>) {
        self.cursor_index = index;
    }

    pub fn get_playing_index(&self) -> Option<usize> {
        self.playing_index
    }
    pub fn set_playing_index(&mut self, index: Option<usize>) {
        self.playing_index = index;
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn list_ref(&self) -> &Vec<Song> {
        &self.list
    }
    pub fn list_mut(&mut self) -> &mut Vec<Song> {
        &mut self.list
    }
}

impl std::default::Default for FilePlaylist {
    fn default() -> Self {
        Self {
            list: Vec::new(),
            cursor_index: None,
            playing_index: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DirectoryPlaylist {
    _list: Vec<PathBuf>,
    pub index: usize,
}

impl DirectoryPlaylist {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_playing_index(&mut self, index: usize) {
        self.index = index;
    }
    pub fn get_playing_index(&self) -> usize {
        self.index
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

impl std::default::Default for DirectoryPlaylist {
    fn default() -> Self {
        Self {
            _list: Vec::new(),
            index: 0,
        }
    }
}
