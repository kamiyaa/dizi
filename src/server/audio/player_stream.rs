use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use rodio::{OutputStream, OutputStreamHandle, Sink};

use dizi_lib::error::DiziResult;

use crate::audio::Song;

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play(Song),
    Pause,
    Resume,
    GetVolume,
    SetVolume(f32),
    GetLen,
}

#[derive(Clone, Debug)]
pub enum PlayerResponse {
    Ok,
    Len(usize),
    Volume(f32),
}

pub struct PlayerStream {
    pub stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
    pub sink: Option<Sink>,
    pub player_res_tx: mpsc::Sender<DiziResult<PlayerResponse>>,
    pub player_req_rx: mpsc::Receiver<PlayerRequest>,
}

impl PlayerStream {
    pub fn new(
        stream: OutputStream,
        stream_handle: OutputStreamHandle,
        player_res_tx: mpsc::Sender<DiziResult<PlayerResponse>>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        Self {
            stream,
            stream_handle,
            sink: None,
            player_res_tx,
            player_req_rx,
        }
    }

    pub fn player_req(&self) -> &mpsc::Receiver<PlayerRequest> {
        &self.player_req_rx
    }

    pub fn player_res(&self) -> &mpsc::Sender<DiziResult<PlayerResponse>> {
        &self.player_res_tx
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<()> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let sink = self.stream_handle.play_once(buffer)?;

        self.sink = Some(sink);
        Ok(())
    }

    pub fn pause(&mut self) {
        if let Some(sink) = self.sink.as_ref() {
            sink.pause();
        }
    }
    pub fn resume(&mut self) {
        if let Some(sink) = self.sink.as_ref() {
            sink.play();
        }
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

    pub fn len(&self) -> usize {
        if let Some(sink) = self.sink.as_ref() {
            sink.len()
        } else {
            0
        }
    }
}

pub fn player_stream_thread(
    player_res_tx: mpsc::Sender<DiziResult<PlayerResponse>>,
    player_req_rx: mpsc::Receiver<PlayerRequest>,
) -> thread::JoinHandle<DiziResult<()>> {
    let handle = thread::spawn(move || {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let mut player_stream =
            PlayerStream::new(stream, stream_handle, player_res_tx, player_req_rx);
        while let Ok(msg) = player_stream.player_req().recv() {
            match msg {
                PlayerRequest::Play(song) => {
                    match player_stream.play(song.file_path()) {
                        Ok(()) => player_stream.player_res().send(Ok(PlayerResponse::Ok)),
                        Err(e) => player_stream.player_res().send(Err(e)),
                    };
                }
                PlayerRequest::Pause => {
                    player_stream.pause();
                    player_stream.player_res().send(Ok(PlayerResponse::Ok));
                }
                PlayerRequest::Resume => {
                    player_stream.resume();
                    player_stream.player_res().send(Ok(PlayerResponse::Ok));
                }
                PlayerRequest::GetVolume => {
                    let volume = player_stream.get_volume();
                    player_stream
                        .player_res()
                        .send(Ok(PlayerResponse::Volume(volume)));
                }
                PlayerRequest::SetVolume(volume) => {
                    player_stream.set_volume(volume);
                    player_stream.player_res().send(Ok(PlayerResponse::Ok));
                }
                PlayerRequest::GetLen => {
                    let len = player_stream.len();
                    player_stream
                        .player_res()
                        .send(Ok(PlayerResponse::Len(len)));
                }

                s => {
                    eprintln!("Not implemented '{:?}'", s);
                }
            }
        }
        Ok(())
    });
    handle
}
