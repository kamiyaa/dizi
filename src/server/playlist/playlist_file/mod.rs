mod impl_ordered_playlist;
mod impl_shuffle_playlist;

pub use impl_ordered_playlist::*;
pub use impl_shuffle_playlist::*;

use std::io;
use std::path::Path;

use dizi_lib::playlist::FilePlaylist;
use dizi_lib::song::Song;

#[derive(Clone, Debug)]
pub struct PlaylistFile {
    _playlist_content: Vec<Song>,
    _playlist_order: Vec<usize>,
    _playlist_index: Option<usize>,
    _shuffle: bool,
}

impl PlaylistFile {
    pub fn new(songs: Vec<Song>) -> Self {
        let content_count = songs.len();
        Self {
            _playlist_content: songs,
            _playlist_order: (0..content_count).collect(),
            _playlist_index: None,
            _shuffle: false,
        }
    }

    pub fn from_file(cwd: &Path, path: &Path) -> io::Result<PlaylistFile> {
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
        let playlist = PlaylistFile::new(songs);
        Ok(playlist)
    }

    pub fn clone_file_playlist(&self) -> FilePlaylist {
        let playing_index = self
            .get_playlist_index()
            .map(|i| self.playlist_order_ref()[i]);
        FilePlaylist {
            list: self._playlist_content.clone(),
            cursor_index: None,
            playing_index,
        }
    }

    fn playlist_content_ref(&self) -> &Vec<Song> {
        &self._playlist_content
    }
    fn playlist_content_mut(&mut self) -> &mut Vec<Song> {
        &mut self._playlist_content
    }

    fn playlist_order_ref(&self) -> &Vec<usize> {
        &self._playlist_order
    }
    fn playlist_order_mut(&mut self) -> &mut Vec<usize> {
        &mut self._playlist_order
    }

    pub fn get_playlist_index(&self) -> Option<usize> {
        self._playlist_index.clone()
    }
    pub fn set_playlist_index(&mut self, index: Option<usize>) {
        self._playlist_index = index;
    }
}
