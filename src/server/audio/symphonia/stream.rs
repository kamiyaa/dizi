use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::Stream;
use log::{debug, log_enabled, Level};

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::audio::request::PlayerRequest;
use crate::events::{ServerEvent, ServerEventSender};

use super::decode::{decode_packets, stream_loop_f32, stream_loop_i16, stream_loop_u16};
#[derive(Clone, Copy, Debug)]

pub enum StreamEvent {
    StreamEnded,
}

#[derive(Clone, Debug)]
pub enum PlayerStreamEvent {
    Stream(StreamEvent),
    Player(PlayerRequest),
}

#[derive(Debug)]
pub struct PlayerStreamEventPoller {
    pub stream_tx: mpsc::Sender<StreamEvent>,
    pub player_res_tx: mpsc::Sender<DiziResult>,
    pub event_tx: mpsc::Sender<PlayerStreamEvent>,
    event_rx: mpsc::Receiver<PlayerStreamEvent>,
}

impl PlayerStreamEventPoller {
    pub fn new(
        player_res_tx: mpsc::Sender<DiziResult>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        Self::init(player_res_tx, player_req_rx)
    }

    fn init(
        player_res_tx: mpsc::Sender<DiziResult>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        let (stream_tx, stream_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        let event_tx_clone = event_tx.clone();
        let _ = thread::spawn(move || loop {
            if let Ok(event) = stream_rx.recv() {
                let _ = event_tx_clone.send(PlayerStreamEvent::Stream(event));
            }
        });

        let event_tx_clone = event_tx.clone();
        let _ = thread::spawn(move || loop {
            if let Ok(req) = player_req_rx.recv() {
                let _ = event_tx_clone.send(PlayerStreamEvent::Player(req));
            }
        });

        Self {
            stream_tx,
            player_res_tx,
            event_tx,
            event_rx,
        }
    }

    pub fn next(&self) -> Result<PlayerStreamEvent, mpsc::RecvError> {
        self.event_rx.recv()
    }

    pub fn player_res(&self) -> &mpsc::Sender<DiziResult> {
        &self.player_res_tx
    }
}

pub struct PlayerStreamState {
    pub stream: Stream,
    pub stream_progress_thread: JoinHandle<()>,
    pub stream_progress_tx: mpsc::Sender<PlayerRequest>,
    pub playback_loop_tx: mpsc::Sender<PlayerRequest>,
}

pub struct PlayerStream {
    event_tx: ServerEventSender,
    event_poller: PlayerStreamEventPoller,
    device: cpal::Device,
    state: Option<PlayerStreamState>,
}

impl PlayerStream {
    pub fn new(
        event_tx: ServerEventSender,
        player_res_tx: mpsc::Sender<DiziResult>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
        device: cpal::Device,
    ) -> Self {
        let event_poller = PlayerStreamEventPoller::new(player_res_tx, player_req_rx);
        Self {
            event_tx,
            event_poller,
            device,
            state: None,
        }
    }

    pub fn pause(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(state) = self.state.as_ref() {
            let _ = state.stream.pause();
            let _ = state.stream_progress_tx.send(PlayerRequest::Pause);
        }
        Ok(())
    }
    pub fn resume(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(state) = self.state.as_ref() {
            let _ = state.stream.play();
            let _ = state.stream_progress_tx.send(PlayerRequest::Resume);
        }
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(state) = self.state.take() {
            let _ = state.stream_progress_thread.join();
        }
        Ok(())
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Some(state) = self.state.as_ref() {
            let _ = state
                .playback_loop_tx
                .send(PlayerRequest::SetVolume(volume));
        }
    }

    pub fn listen_for_events(&mut self) -> DiziResult {
        while let Ok(msg) = self.event_poller.next() {
            match msg {
                PlayerStreamEvent::Player(req) => match req {
                    PlayerRequest::Play(song) => {
                        let stream_res = self.play(song.file_path());
                        match stream_res {
                            Ok(stream_res) => {
                                let (stream, playback_loop_tx) = stream_res;
                                let (handle, tx) = self.init_stream_progress_thread();
                                self.state = Some(PlayerStreamState {
                                    stream,
                                    stream_progress_thread: handle,
                                    stream_progress_tx: tx,
                                    playback_loop_tx,
                                });
                                self.event_poller.player_res().send(Ok(()))?;
                            }
                            Err(e) => self.event_poller.player_res().send(Err(e))?,
                        };
                    }
                    PlayerRequest::Pause => {
                        self.pause()?;
                        self.event_poller.player_res().send(Ok(()))?;
                    }
                    PlayerRequest::Stop => {
                        self.stop()?;
                        self.event_poller.player_res().send(Ok(()))?;
                    }
                    PlayerRequest::Resume => {
                        self.resume()?;
                        self.event_poller.player_res().send(Ok(()))?;
                    }
                    PlayerRequest::SetVolume(volume) => {
                        self.set_volume(volume);
                        self.event_poller.player_res().send(Ok(()))?;
                    }
                },
                PlayerStreamEvent::Stream(event) => match event {
                    StreamEvent::StreamEnded => {
                        self.stop()?;
                        self.event_tx.send(ServerEvent::PlayerDone)?;
                    }
                },
            }
        }
        Ok(())
    }

    fn init_stream_progress_thread(&mut self) -> (JoinHandle<()>, mpsc::Sender<PlayerRequest>) {
        const POLL_RATE: Duration = Duration::from_secs(1);

        let (stream_progress_tx, stream_progress_rx) = mpsc::channel();
        let event_tx_clone = self.event_tx.clone();
        let stream_progress_thread = thread::spawn(move || {
            for i in 0.. {
                let duration_played = Duration::from_secs(i);
                let _ = event_tx_clone.send(ServerEvent::PlayerProgressUpdate(duration_played));
                if log_enabled!(Level::Debug) {
                    debug!("{:?}", duration_played);
                }

                match stream_progress_rx.recv_timeout(POLL_RATE) {
                    Ok(msg) => match msg {
                        PlayerRequest::Pause => {
                            while let Ok(msg) = stream_progress_rx.recv() {
                                match msg {
                                    PlayerRequest::Resume => break,
                                    _ => {}
                                }
                            }
                        }
                        PlayerRequest::Resume => {}
                        _ => {}
                    },
                    Err(RecvTimeoutError::Timeout) => {}
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
        });
        (stream_progress_thread, stream_progress_tx)
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<(Stream, mpsc::Sender<PlayerRequest>)> {
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        };

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let file = std::fs::File::open(path)?;

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(file), Default::default());
        let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

        // Get the instantiated format reader.
        let track = probed
            .format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL);
        match track {
            Some(track) => {
                // Store the track identifier, it will be used to filter packets.
                let track_id = track.id;

                // Use the default options for the decoder.
                let dec_opts: DecoderOptions = Default::default();

                // Create a decoder for the track.
                let decoder =
                    symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)?;

                let config = self.device.default_output_config().unwrap();

                let stream_tx = self.event_poller.stream_tx.clone();

                match config.sample_format() {
                    cpal::SampleFormat::F32 => {
                        let packets = decode_packets::<f32>(probed.format, decoder, track_id);
                        match packets {
                            Some(packets) => {
                                let res = stream_loop_f32(
                                    stream_tx,
                                    &self.device,
                                    &config.into(),
                                    packets,
                                )?;
                                Ok(res)
                            }
                            None => Err(DiziError::new(
                                DiziErrorKind::NoDevice,
                                "Error eading packets".to_string(),
                            )),
                        }
                    }
                    cpal::SampleFormat::I16 => {
                        let packets = decode_packets::<i16>(probed.format, decoder, track_id);
                        match packets {
                            Some(packets) => {
                                let res = stream_loop_i16(
                                    stream_tx,
                                    &self.device,
                                    &config.into(),
                                    packets,
                                )?;
                                Ok(res)
                            }
                            None => Err(DiziError::new(
                                DiziErrorKind::NoDevice,
                                "Error eading packets".to_string(),
                            )),
                        }
                    }
                    cpal::SampleFormat::U16 => {
                        let packets = decode_packets::<u16>(probed.format, decoder, track_id);
                        match packets {
                            Some(packets) => {
                                let res = stream_loop_u16(
                                    stream_tx,
                                    &self.device,
                                    &config.into(),
                                    packets,
                                )?;
                                Ok(res)
                            }
                            None => Err(DiziError::new(
                                DiziErrorKind::NoDevice,
                                "Error eading packets".to_string(),
                            )),
                        }
                    }
                }
            }
            None => Err(DiziError::new(DiziErrorKind::NoDevice, "".to_string())),
        }
    }
}
