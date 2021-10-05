use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::thread;

use rodio::{Decoder, OutputStream, OutputStreamHandle};
use rodio::source::Source;

use dizi_commands::error::DiziResult;
use crate::audio::Song;

#[derive(Clone, Debug)]
pub enum PlayerStatus {
    Playing(Song),
    Paused(Song),
    Stopped,
}

#[derive(Clone, Debug)]
pub struct Player {
//    pipewire: PipewireData,
    status: PlayerStatus,
    shuffle: bool,
    repeat: bool,
    next: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            status: PlayerStatus::Stopped,
            shuffle: false,
            repeat: true,
            next: true,
        }
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<thread::JoinHandle<DiziResult<()>>> {
        let song = Song::new(path)?;

        let path_clone = path.to_path_buf();

        let handle = thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let file = File::open(&path_clone)?;
            let buffer = BufReader::new(file);

            let sink = stream_handle.play_once(buffer)?;
            sink.sleep_until_end();
            Ok(())
        });

        self.status = PlayerStatus::Playing(song);
        Ok(handle)
    }
}

/*
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let pipewire = PipewireData::new()?;

        Ok(Self {
            current_song: None,
            pipewire,
        })
    }

    pub fn play() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
*/
