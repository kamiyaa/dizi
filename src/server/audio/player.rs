use std::path::Path;
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;
use dizi_lib::song::Song;

use crate::audio::{player_stream_thread, PlayerRequest, PlayerResponse, Playlist};

#[derive(Copy, Clone, Debug, PartialEq)]
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
    // event_tx: mpsc::Sender<PlayerResponse>,
    playlist: Playlist,
    player_handle: thread::JoinHandle<DiziResult<()>>,
    player_req_tx: mpsc::Sender<PlayerRequest>,
    player_res_rx: mpsc::Receiver<DiziResult<PlayerResponse>>,
}

impl Player {
    pub fn new() -> Self {
        let (player_req_tx, player_req_rx) = mpsc::channel();
        let (player_res_tx, player_res_rx) = mpsc::channel();

        let player_handle = player_stream_thread(player_res_tx, player_req_rx);

        Self {
            status: PlayerStatus::Stopped,
            shuffle: false,
            repeat: true,
            next: true,
            current_song: None,
            playlist: Playlist::new(),
            player_handle,
            player_req_tx,
            player_res_rx,
        }
    }

    fn player_stream_req(&self) -> &mpsc::Sender<PlayerRequest> {
        &self.player_req_tx
    }

    fn player_stream_res(&self) -> &mpsc::Receiver<DiziResult<PlayerResponse>> {
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

    pub fn toggle_play(&mut self) -> DiziResult<PlayerStatus> {
        match self.status {
            PlayerStatus::Playing => {
                self.pause();
                Ok(PlayerStatus::Paused)
            }
            PlayerStatus::Paused => {
                self.resume();
                Ok(PlayerStatus::Playing)
            }
            _ => Ok(PlayerStatus::Stopped),
        }
    }

    pub fn get_volume(&self) -> DiziResult<f32> {
        self.player_stream_req().send(PlayerRequest::GetVolume);

        match self.player_stream_res().recv().map(|r| r.unwrap()) {
            Ok(PlayerResponse::Volume(volume)) => Ok(volume),
            _ => Ok(0.0),
        }
    }

    pub fn set_volume(&self, volume: f32) -> DiziResult<()> {
        self.player_stream_req()
            .send(PlayerRequest::SetVolume(volume));

        match self.player_stream_res().recv().map(|r| r.unwrap()) {
            _ => Ok(()),
        }
    }

    pub fn len(&self) -> DiziResult<usize> {
        self.player_stream_req().send(PlayerRequest::GetLen);

        match self.player_stream_res().recv().map(|r| r.unwrap()) {
            Ok(PlayerResponse::Len(u)) => Ok(u),
            _ => Ok(0),
        }
    }

    pub fn is_paused(&self) -> bool {
        self.status != PlayerStatus::Playing
    }
}
