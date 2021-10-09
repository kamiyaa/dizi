use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc;

use rodio::{OutputStream, OutputStreamHandle, Sink};

use dizi_commands::api_command::ApiCommand;
use dizi_commands::error::DiziResult;

use crate::audio::Song;

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play(Song),
    Pause,
    Resume,
    GetVolume,
    SetVolume(f32),
}

#[derive(Clone, Debug)]
pub enum PlayerResponse {
    Ok,
    Volume(f32),
}

pub struct PlayerStream {
    pub stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    pub sink: Option<Sink>,
    pub event_tx: mpsc::Sender<DiziResult<PlayerResponse>>,
    pub event_rx: mpsc::Receiver<PlayerRequest>,
}

impl PlayerStream {
    pub fn new(
        stream: OutputStream,
        stream_handle: OutputStreamHandle,
        event_tx: mpsc::Sender<DiziResult<PlayerResponse>>,
        event_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        Self {
            stream,
            stream_handle,
            sink: None,
            event_tx,
            event_rx,
        }
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<PlayerResponse> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let sink = self.stream_handle.play_once(buffer)?;

        self.sink = Some(sink);
        Ok(PlayerResponse::Ok)
    }

    pub fn pause(&mut self) -> DiziResult<PlayerResponse> {
        if let Some(sink) = self.sink.as_ref() {
            sink.pause();
        }
        Ok(PlayerResponse::Ok)
    }
    pub fn resume(&mut self) -> DiziResult<PlayerResponse> {
        if let Some(sink) = self.sink.as_ref() {
            sink.play();
        }
        Ok(PlayerResponse::Ok)
    }

    pub fn get_volume(&self) -> f32 {
        if let Some(sink) = self.sink.as_ref() {
            sink.volume()
        } else {
            0.0
        }
    }

    pub fn set_volume(&self, volume: f32) {
        if let Some(sink) = self.sink.as_ref() {
            sink.set_volume(volume);
        }
    }
}
