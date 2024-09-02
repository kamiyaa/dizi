mod impl_playlist;

use std::fs;
use std::io;
use std::path::Path;

use dizi::playlist::FilePlaylist;
use dizi::playlist::PlaylistType;
use dizi::song::{DiziFile, DiziSongEntry};

#[derive(Clone, Debug)]
pub struct DiziPlaylist {
    pub contents: Vec<DiziSongEntry>,
    pub order: Vec<usize>,
    pub order_index: Option<usize>,
    pub playlist_type: PlaylistType,
}

impl DiziPlaylist {
    pub fn new(contents: Vec<DiziSongEntry>, playlist_type: PlaylistType) -> Self {
        let content_count = contents.len();
        Self {
            contents,
            order: (0..content_count).collect(),
            order_index: None,
            playlist_type,
        }
    }

    pub fn from_dir(path: &Path) -> io::Result<Self> {
        // only process regular files
        // if we can't read it, then don't play it
        let mut contents: Vec<_> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|p| p.is_file())
            .map(|path| DiziSongEntry::Unloaded(DiziFile::new(&path)))
            .collect();
        contents.sort_by(|a, b| a.file_name().cmp(b.file_name()));

        let len = contents.len();
        Ok(Self {
            contents,
            order: (0..len).collect(),
            order_index: None,
            playlist_type: PlaylistType::DirectoryListing,
        })
    }

    pub fn from_file(cwd: &Path, path: &Path) -> io::Result<DiziPlaylist> {
        let mut reader = m3u::Reader::open(path)?;
        let read_playlist: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
        let mut entries = Vec::new();
        for entry in read_playlist {
            if let m3u::Entry::Path(p) = entry {
                let file_path = if p.is_absolute() {
                    p
                } else {
                    let mut new_path = cwd.to_path_buf();
                    new_path.push(p);
                    new_path
                };
                let entry = DiziSongEntry::Unloaded(DiziFile::new(&file_path));
                entries.push(entry);
            }
        }
        let playlist = DiziPlaylist::new(entries, PlaylistType::PlaylistFile);
        Ok(playlist)
    }

    pub fn to_file_playlist(&self) -> FilePlaylist {
        let playing_index = self.order_index.and_then(|i| self.order.get(i)).map(|i| *i);
        FilePlaylist {
            list: self.contents.clone(),
            cursor_index: None,
            playing_index,
        }
    }
}

impl std::default::Default for DiziPlaylist {
    fn default() -> Self {
        Self {
            contents: Vec::new(),
            order: Vec::new(),
            order_index: None,
            playlist_type: PlaylistType::PlaylistFile,
        }
    }
}
