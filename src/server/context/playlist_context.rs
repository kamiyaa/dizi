use dizi_lib::playlist::PlaylistType;
use dizi_lib::song::Song;

use crate::playlist::playlist_directory::PlaylistDirectory;
use crate::playlist::playlist_file::PlaylistFile;
use crate::playlist::traits::{OrderedPlaylist, OrderedPlaylistEntry};

#[derive(Clone, Debug)]
pub struct PlaylistContext {
    pub file_playlist: PlaylistFile,
    pub directory_playlist: PlaylistDirectory,
    pub _type: PlaylistType,
}

impl PlaylistContext {
    pub fn get_type(&self) -> PlaylistType {
        self._type
    }
    pub fn set_type(&mut self, _type: PlaylistType) {
        self._type = _type;
    }

    pub fn file_playlist_ref(&self) -> &PlaylistFile {
        &self.file_playlist
    }
    pub fn file_playlist_mut(&mut self) -> &mut PlaylistFile {
        &mut self.file_playlist
    }

    pub fn directory_playlist_ref(&self) -> &PlaylistDirectory {
        &self.directory_playlist
    }
    pub fn directory_playlist_mut(&mut self) -> &mut PlaylistDirectory {
        &mut self.directory_playlist
    }

    pub fn play(&mut self, index: usize) {}

    pub fn get_entry(&self, index: usize) -> &Song {
        match self.get_type() {
            PlaylistType::PlaylistFile => self.file_playlist.get_entry(index),
            PlaylistType::DirectoryListing => self.directory_playlist.get_entry(index),
        }
    }

    pub fn get_current_entry(&self) -> Option<OrderedPlaylistEntry> {
        match self.get_type() {
            PlaylistType::PlaylistFile => self.file_playlist.get_current_entry(),
            PlaylistType::DirectoryListing => self.directory_playlist.get_current_entry(),
        }
    }

    pub fn next_song(&mut self) -> Option<OrderedPlaylistEntry> {
        match self.get_type() {
            PlaylistType::PlaylistFile => self.file_playlist.next_song(),
            PlaylistType::DirectoryListing => self.directory_playlist.next_song(),
        }
    }

    pub fn previous_song(&mut self) -> Option<OrderedPlaylistEntry> {
        match self.get_type() {
            PlaylistType::PlaylistFile => self.file_playlist.previous_song(),
            PlaylistType::DirectoryListing => self.directory_playlist.previous_song(),
        }
    }

    pub fn is_end(&self) -> bool {
        match self.get_type() {
            PlaylistType::PlaylistFile => self.file_playlist.is_end(),
            PlaylistType::DirectoryListing => self.directory_playlist.is_end(),
        }
    }
}

impl std::default::Default for PlaylistContext {
    fn default() -> Self {
        Self {
            file_playlist: PlaylistFile::new(Vec::new()),
            directory_playlist: PlaylistDirectory::new(Vec::new()),
            _type: PlaylistType::PlaylistFile,
        }
    }
}
