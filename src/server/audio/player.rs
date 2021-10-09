use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

use rodio::source::Source;
use rodio::{Decoder, OutputStream, OutputStreamHandle};

use crate::audio::{PlayerStream, PlayerStreamMsg, Song};
use dizi_commands::api_command::ApiCommand;
use dizi_commands::error::DiziResult;

#[derive(Copy, Clone, Debug)]
pub enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub struct Player {
    status: PlayerStatus,
    shuffle: bool,
    repeat: bool,
    next: bool,
    current_song: Option<Song>,
    player_handle: thread::JoinHandle<DiziResult<()>>,
    player_stream_tx: mpsc::Sender<PlayerStreamMsg>,
    player_rx: mpsc::Receiver<DiziResult<()>>,
}

impl Player {
    pub fn new() -> Self {
        let (player_tx, player_rx) = mpsc::channel();
        let (player_stream_tx, player_stream_rx) = mpsc::channel();

        let player_handle = thread::spawn(move || {
            let (stream, stream_handle) = OutputStream::try_default()?;
            let mut player_stream =
                PlayerStream::new(stream, stream_handle, player_tx, player_stream_rx);
            while let Ok(msg) = player_stream.event_rx.recv() {
                match msg {
                    PlayerStreamMsg::Play(song) => {
                        player_stream.play(song.file_path());
                        player_stream.event_tx.send(Ok(()));
                    }
                    PlayerStreamMsg::Pause => {
                        player_stream.pause();
                        player_stream.event_tx.send(Ok(()));
                    }
                    PlayerStreamMsg::Resume => {
                        player_stream.resume();
                        player_stream.event_tx.send(Ok(()));
                    }
                }
            }
            Ok(())
        });

        Self {
            status: PlayerStatus::Stopped,
            shuffle: false,
            repeat: true,
            next: true,
            current_song: None,
            player_handle,
            player_stream_tx,
            player_rx,
        }
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<()> {
        let song = Song::new(path)?;

        self.player_stream_tx
            .send(PlayerStreamMsg::Play(song.clone()));

        match self.player_rx.recv().map(|r| r.unwrap()) {
            Ok(res) => {
                self.status = PlayerStatus::Playing;
                self.current_song = Some(song);
            }
            Err(_) => {}
        }
        Ok(())
    }

    pub fn pause(&mut self) -> DiziResult<()> {
        self.player_stream_tx.send(PlayerStreamMsg::Pause);

        match self.player_rx.recv().map(|r| r.unwrap()) {
            Ok(res) => {
                self.status = PlayerStatus::Paused;
            }
            Err(_) => {}
        }
        Ok(())
    }

    pub fn resume(&mut self) -> DiziResult<()> {
        self.player_stream_tx.send(PlayerStreamMsg::Resume);

        match self.player_rx.recv().map(|r| r.unwrap()) {
            Ok(res) => {
                self.status = PlayerStatus::Playing;
            }
            Err(_) => {}
        }
        Ok(())
    }

    pub fn toggle_play(&mut self) -> DiziResult<()> {
        match self.status {
            PlayerStatus::Playing => self.pause(),
            PlayerStatus::Paused => self.resume(),
            _ => Ok(()),
        }
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
