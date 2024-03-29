mod impl_ordered_playlist;
mod impl_shuffle_playlist;

use std::fs;
use std::io;
use std::path::Path;

use dizi::song::Song;

#[derive(Clone, Debug)]
pub struct PlaylistDirectory {
    _playlist_content: Vec<Song>,
    _playlist_order: Vec<usize>,
    _playlist_index: Option<usize>,
}

impl PlaylistDirectory {
    pub fn new(songs: Vec<Song>) -> Self {
        let songs_count = songs.len();
        Self {
            _playlist_content: songs,
            _playlist_order: (0..songs_count).collect(),
            _playlist_index: None,
        }
    }

    pub fn from_path(path: &Path) -> io::Result<Self> {
        // only process regular files
        // if we can't read it, then don't play it
        let mut songs: Vec<Song> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .map(|path| Song::new(&path))
            .collect();
        songs.sort_by(|a, b| a.file_name().cmp(b.file_name()));

        let len = songs.len();
        Ok(Self {
            _playlist_content: songs,
            _playlist_order: (0..len).collect(),
            _playlist_index: None,
        })
    }

    pub fn get_playlist_index(&self) -> Option<usize> {
        self._playlist_index
    }
    pub fn set_playlist_index(&mut self, index: Option<usize>) {
        self._playlist_index = index;
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
}
