use std::path::Path;
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;
use dizi_lib::song::Song;

use crate::audio::{player_stream, PlayerRequest, Playlist};
use crate::config;
use crate::events::ServerEventSender;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug)]
pub struct Player {
    current_song: Option<Song>,

    status: PlayerStatus,
    volume: f32,

    shuffle: bool,
    repeat: bool,
    next: bool,

    event_tx: ServerEventSender,

    // event_tx: mpsc::Sender<PlayerResponse>,
    playlist: Playlist,
    player_handle: thread::JoinHandle<DiziResult<()>>,
    player_req_tx: mpsc::Sender<PlayerRequest>,
    player_res_rx: mpsc::Receiver<DiziResult<()>>,
}

impl Player {
    pub fn new(config_t: &config::AppConfig, event_tx: ServerEventSender) -> Self {
        let (player_req_tx, player_req_rx) = mpsc::channel();
        let (player_res_tx, player_res_rx) = mpsc::channel();

        let config_t2 = config_t.clone();
        let event_tx2 = event_tx.clone();
        let player_handle = thread::spawn(move || {
            player_stream(config_t2, player_res_tx, player_req_rx, event_tx2)
        });

        Self {
            current_song: None,

            status: PlayerStatus::Stopped,
            volume: 1.0,

            shuffle: false,
            repeat: true,
            next: true,

            event_tx,

            playlist: Playlist::new(),
            player_handle,
            player_req_tx,
            player_res_rx,
        }
    }

    fn player_stream_req(&self) -> &mpsc::Sender<PlayerRequest> {
        &self.player_req_tx
    }

    fn player_stream_res(&self) -> &mpsc::Receiver<DiziResult<()>> {
        &self.player_res_rx
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<()> {
        let song = Song::new(path)?;

        self.player_stream_req()
            .send(PlayerRequest::Play(song.clone()));

        let resp = self.player_stream_res().recv();
        match resp {
            Ok(msg) => match msg {
                Ok(_) => {
                    self.status = PlayerStatus::Playing;
                    self.current_song = Some(song);
                }
                Err(e) => {
                    eprintln!("Failed to play song: {:?}", e);
                }
            },
            Err(e) => {
                eprintln!("Failed to receive msg from player stream");
            }
        }
        Ok(())
    }

    pub fn pause(&mut self) -> DiziResult<()> {
        self.player_stream_req().send(PlayerRequest::Pause);

        let _ = self.player_stream_res().recv();
        self.status = PlayerStatus::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> DiziResult<()> {
        self.player_stream_req().send(PlayerRequest::Resume);

        let _ = self.player_stream_res().recv();
        self.status = PlayerStatus::Playing;
        Ok(())
    }

    pub fn play_status(&self) -> PlayerStatus {
        self.status
    }

    pub fn toggle_play(&mut self) -> DiziResult<PlayerStatus> {
        match self.status {
            PlayerStatus::Playing => {
                self.pause()?;
                Ok(PlayerStatus::Paused)
            }
            PlayerStatus::Paused => {
                self.resume()?;
                Ok(PlayerStatus::Playing)
            }
            _ => Ok(PlayerStatus::Stopped),
        }
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, volume: f32) -> DiziResult<()> {
        self.player_stream_req()
            .send(PlayerRequest::SetVolume(volume));

        match self.player_stream_res().recv().map(|r| r.unwrap()) {
            Ok(_) => {
                self.volume = volume;
                Ok(())
            }
            Err(_) => Ok(()),
        }
    }
}
