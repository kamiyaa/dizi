use dizi::playlist::PlaylistType;

use crate::{
    playlist::DiziPlaylist,
    traits::{DiziPlaylistEntry, DiziPlaylistTrait},
};

#[derive(Clone, Debug)]
pub struct PlaylistContext {
    pub file_playlist: DiziPlaylist,
    pub directory_playlist: DiziPlaylist,
    pub current_playlist_type: PlaylistType,
}

impl PlaylistContext {
    pub fn current_playlist_ref(&self) -> &DiziPlaylist {
        match self.current_playlist_type {
            PlaylistType::DirectoryListing => &self.directory_playlist,
            PlaylistType::PlaylistFile => &self.file_playlist,
        }
    }
    pub fn current_playlist_mut(&mut self) -> &mut DiziPlaylist {
        match self.current_playlist_type {
            PlaylistType::DirectoryListing => &mut self.directory_playlist,
            PlaylistType::PlaylistFile => &mut self.file_playlist,
        }
    }

    pub fn current_song(&self) -> Option<DiziPlaylistEntry> {
        match self.current_playlist_type {
            PlaylistType::PlaylistFile => self.file_playlist.current_entry(),
            PlaylistType::DirectoryListing => self.directory_playlist.current_entry(),
        }
    }

    pub fn next_song_peak(&self) -> Option<DiziPlaylistEntry> {
        match self.current_playlist_type {
            PlaylistType::PlaylistFile => self.file_playlist.next_song_peak(),
            PlaylistType::DirectoryListing => self.directory_playlist.next_song_peak(),
        }
    }

    pub fn previous_song_peak(&self) -> Option<DiziPlaylistEntry> {
        match self.current_playlist_type {
            PlaylistType::PlaylistFile => self.file_playlist.previous_song_peak(),
            PlaylistType::DirectoryListing => self.directory_playlist.previous_song_peak(),
        }
    }

    pub fn is_end(&self) -> bool {
        match self.current_playlist_type {
            PlaylistType::PlaylistFile => self.file_playlist.is_end(),
            PlaylistType::DirectoryListing => self.directory_playlist.is_end(),
        }
    }
}

impl std::default::Default for PlaylistContext {
    fn default() -> Self {
        Self {
            file_playlist: DiziPlaylist::default(),
            directory_playlist: DiziPlaylist::default(),
            current_playlist_type: PlaylistType::PlaylistFile,
        }
    }
}
