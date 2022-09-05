use std::path::{Path, PathBuf};

use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use cpal::traits::HostTrait;

use log::{debug, log_enabled, Level};

use rodio::queue;
use rodio::source::{Amplify, Pausable, Source, Stoppable};
use rodio::{Decoder, OutputStream};

use dizi_lib::error::DiziResult;

use crate::audio::request::PlayerRequest;
use crate::config;
use crate::events::{ServerEvent, ServerEventSender};

pub struct PlayerStream {
    event_tx: ServerEventSender,
    player_res_tx: mpsc::Sender<DiziResult>,
    player_req_rx: mpsc::Receiver<PlayerRequest>,
}

impl PlayerStream {
    pub fn init(
        event_tx: ServerEventSender,
        player_res_tx: mpsc::Sender<DiziResult>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        Self {
            event_tx,
            player_res_tx,
            player_req_rx,
        }
    }

    pub fn player_req(&self) -> &mpsc::Receiver<PlayerRequest> {
        &self.player_req_rx
    }

    pub fn player_res(&self) -> &mpsc::Sender<DiziResult> {
        &self.player_res_tx
    }

    pub fn pause(&self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        Ok(())
    }
    pub fn resume(&self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        Ok(())
    }

    pub fn set_volume(&mut self, volume: f32) {}

    pub fn play(&mut self, path: &Path) -> DiziResult {
        Ok(())
    }
}

pub fn init_player_stream(
    player_res_tx: mpsc::Sender<DiziResult>,
    player_req_rx: mpsc::Receiver<PlayerRequest>,
    event_tx: ServerEventSender,
) -> DiziResult {
    let mut player_stream = PlayerStream::init(event_tx, player_res_tx, player_req_rx);

    let stream_listeners: Arc<Mutex<Vec<ServerEventSender>>> = Arc::new(Mutex::new(vec![]));
    let mut done_listener: Option<thread::JoinHandle<()>> = None;

    while let Ok(msg) = player_stream.player_req().recv() {
        match msg {
            PlayerRequest::Play(song) => {
                // Before playing new song, make sure to clear any listeners waiting for previous
                // song to finish. This prevents a loop where new song triggers the end of previous
                // song which triggers a new song, and repeat.
                match stream_listeners.lock() {
                    Ok(mut vec) => vec.clear(),
                    _ => {}
                }

                let res = player_stream.play(song.file_path());

                match res {
                    Ok(receiver) => {}
                    Err(e) => player_stream.player_res().send(Err(e))?,
                };
            }
            PlayerRequest::Pause => {
                player_stream.pause()?;
                player_stream.player_res().send(Ok(()))?;
            }
            PlayerRequest::Stop => {
                player_stream.stop()?;
                player_stream.player_res().send(Ok(()))?;
            }
            PlayerRequest::Resume => {
                player_stream.resume()?;
                player_stream.player_res().send(Ok(()))?;
            }
            PlayerRequest::SetVolume(volume) => {
                player_stream.set_volume(volume);
                player_stream.player_res().send(Ok(()))?;
            }
        }
    }
    Ok(())
}
