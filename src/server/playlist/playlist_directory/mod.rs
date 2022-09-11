mod impl_ordered_playlist;
mod impl_shuffle_playlist;

pub use impl_ordered_playlist::*;
pub use impl_shuffle_playlist::*;

use std::fs;
use std::io;
use std::path::Path;

use dizi_lib::song::Song;

#[derive(Clone, Debug)]
pub struct PlaylistDirectory {
    _playlist_content: Vec<Song>,
    _playlist_order: Vec<usize>,
    _playlist_index: Option<usize>,
    _shuffle: bool,
}

impl PlaylistDirectory {
    pub fn new(songs: Vec<Song>) -> Self {
        let songs_count = songs.len();
        Self {
            _playlist_content: songs,
            _playlist_order: (0..songs_count).collect(),
            _playlist_index: None,
            _shuffle: false,
        }
    }

    pub fn from_path(path: &Path) -> io::Result<Self> {
        // only process regular files
        // if we can't read it, then don't play it
        let songs: Vec<Song> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .map(|path| Song::new(&path))
            .collect();

        let len = songs.len();
        Ok(Self {
            _playlist_content: songs,
            _playlist_order: (0..len).collect(),
            _playlist_index: None,
            _shuffle: false,
        })
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
        self._playlist_index
    }
    pub fn set_playlist_index(&mut self, index: Option<usize>) {
        self._playlist_index = index;
    }
}
