use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::song::DiziSongEntry;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlaylistType {
    DirectoryListing,
    PlaylistFile,
}

impl ToString for PlaylistType {
    fn to_string(&self) -> String {
        match *self {
            Self::DirectoryListing => "directory".to_string(),
            Self::PlaylistFile => "file".to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilePlaylist {
    pub list: Vec<DiziSongEntry>,
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

    pub fn playlist(&self) -> &[DiziSongEntry] {
        self.list.as_slice()
    }

    pub fn clear(&mut self) {
        self.list_mut().clear();
        self.cursor_index = None;
        self.playing_index = None;
    }

    pub fn append_song(&mut self, s: DiziSongEntry) {
        self.list_mut().push(s);
    }

    pub fn remove_song(&mut self, index: usize) -> DiziSongEntry {
        let song = self.list_mut().remove(index);

        if let Some(playing_index) = self.playing_index {
            if playing_index == index {
                self.set_playing_index(None);
            }
        }
        if self.list_ref().is_empty() {
            self.set_cursor_index(None);
        } else {
            match self.get_cursor_index() {
                Some(i) if i >= self.list_ref().len() => {
                    self.set_cursor_index(Some(self.list_ref().len() - 1));
                }
                _ => {}
            }
            match self.get_playing_index() {
                Some(i) if i > index => {
                    self.set_playing_index(Some(i - 1));
                }
                _ => {}
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

    pub fn list_ref(&self) -> &[DiziSongEntry] {
        &self.list
    }
    pub fn list_mut(&mut self) -> &mut Vec<DiziSongEntry> {
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
