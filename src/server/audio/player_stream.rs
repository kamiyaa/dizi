use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc;

use rodio::{OutputStream, OutputStreamHandle, Sink};

use dizi_commands::api_command::ApiCommand;
use dizi_commands::error::DiziResult;

use crate::audio::Song;

pub enum PlayerStreamMsg {
    Play(Song),
    Pause,
    Resume,
}

pub struct PlayerStream {
    pub stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    pub sink: Option<Sink>,
    pub event_tx: mpsc::Sender<DiziResult<()>>,
    pub event_rx: mpsc::Receiver<PlayerStreamMsg>,
}

impl PlayerStream {
    pub fn new(
        stream: OutputStream,
        stream_handle: OutputStreamHandle,
        event_tx: mpsc::Sender<DiziResult<()>>,
        event_rx: mpsc::Receiver<PlayerStreamMsg>,
    ) -> Self {
        Self {
            stream,
            stream_handle,
            sink: None,
            event_tx,
            event_rx,
        }
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<()> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let sink = self.stream_handle.play_once(buffer)?;

        self.sink = Some(sink);
        Ok(())
    }

    pub fn pause(&mut self) -> DiziResult<()> {
        if let Some(sink) = self.sink.as_ref() {
            sink.pause();
        }
        Ok(())
    }
    pub fn resume(&mut self) -> DiziResult<()> {
        if let Some(sink) = self.sink.as_ref() {
            sink.play();
        }
        Ok(())
    }
}
