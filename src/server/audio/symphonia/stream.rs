use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc::RecvTimeoutError;
use std::sync::{mpsc, RwLock};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatOptions, FormatReader, Track};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleRate, Stream, StreamConfig};
use log::{debug, log_enabled, Level};

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::audio::device::get_default_host;
use crate::audio::request::PlayerRequest;
use crate::config;
use crate::events::{ServerEvent, ServerEventSender};
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

pub struct PlayerStream {
    event_tx: ServerEventSender,
    event_poller: PlayerStreamEventPoller,
    device: cpal::Device,
    stream: Option<Stream>,
    stream_progress_thread: Option<JoinHandle<()>>,
    stream_progress_tx: Option<mpsc::Sender<PlayerRequest>>,
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
            stream: None,
            stream_progress_thread: None,
            stream_progress_tx: None,
        }
    }

    pub fn pause(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(stream) = self.stream.as_ref() {
            let _ = stream.pause();
        }
        if let Some(stream_progress_tx) = self.stream_progress_tx.as_ref() {
            let _ = stream_progress_tx.send(PlayerRequest::Pause);
        }
        Ok(())
    }
    pub fn resume(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        if let Some(stream) = self.stream.as_ref() {
            let _ = stream.play();
        }
        if let Some(stream_progress_tx) = self.stream_progress_tx.as_ref() {
            let _ = stream_progress_tx.send(PlayerRequest::Resume);
        }
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), mpsc::SendError<PlayerRequest>> {
        self.stream_progress_tx.take();
        self.stream.take();
        if let Some(handle) = self.stream_progress_thread.take() {
            handle.join();
        }
        Ok(())
    }

    pub fn set_volume(&mut self, volume: f32) {}

    pub fn listen_for_events(&mut self) -> DiziResult {
        let stream_listeners: Arc<Mutex<Vec<ServerEventSender>>> = Arc::new(Mutex::new(vec![]));
        let mut done_listener: Option<thread::JoinHandle<()>> = None;

        while let Ok(msg) = self.event_poller.next() {
            match msg {
                PlayerStreamEvent::Player(req) => match req {
                    PlayerRequest::Play(song) => {
                        let stream_res = self.play(song.file_path());
                        match stream_res {
                            Ok(stream_res) => {
                                self.stream = Some(stream_res);
                                self.init_stream_progress_thread();
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
                        self.stop();
                        self.event_tx.send(ServerEvent::PlayerDone)?;
                    }
                },
            }
        }
        Ok(())
    }

    fn init_stream_progress_thread(&mut self) {
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
        self.stream_progress_thread = Some(stream_progress_thread);
        self.stream_progress_tx = Some(stream_progress_tx);
    }

    pub fn play(&mut self, path: &Path) -> DiziResult<Stream> {
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

                let res = match config.sample_format() {
                    cpal::SampleFormat::F32 => Self::decode_loop::<f32>(
                        stream_tx,
                        &self.device,
                        &config.into(),
                        probed.format,
                        decoder,
                        track_id,
                    ),
                    cpal::SampleFormat::I16 => Self::decode_loop::<i16>(
                        stream_tx,
                        &self.device,
                        &config.into(),
                        probed.format,
                        decoder,
                        track_id,
                    ),
                    cpal::SampleFormat::U16 => Self::decode_loop::<u16>(
                        stream_tx,
                        &self.device,
                        &config.into(),
                        probed.format,
                        decoder,
                        track_id,
                    ),
                };
                res
            }
            None => Err(DiziError::new(DiziErrorKind::NoDevice, "".to_string())),
        }
    }

    fn decode_loop<T>(
        stream_tx: mpsc::Sender<StreamEvent>,
        device: &cpal::Device,
        config: &StreamConfig,
        mut format: Box<dyn FormatReader>,
        mut decoder: Box<dyn Decoder>,
        track_id: u32,
    ) -> DiziResult<Stream>
    where
        T: symphonia::core::sample::Sample
            + cpal::Sample
            + std::marker::Send
            + 'static
            + symphonia::core::conv::FromSample<i8>
            + symphonia::core::conv::FromSample<i16>
            + symphonia::core::conv::FromSample<i32>
            + symphonia::core::conv::FromSample<u8>
            + symphonia::core::conv::FromSample<u16>
            + symphonia::core::conv::FromSample<u32>
            + symphonia::core::conv::FromSample<f32>
            + symphonia::core::conv::FromSample<f64>
            + symphonia::core::conv::FromSample<symphonia::core::sample::i24>
            + symphonia::core::conv::FromSample<symphonia::core::sample::u24>,
    {
        let mut channel_data: Option<(usize, Vec<T>)> = None;

        // The decode loop.
        loop {
            // Get the next packet from the media format.
            let packet = match format.next_packet() {
                Ok(packet) => packet,
                Err(SymphoniaError::ResetRequired) => {
                    // The track list has been changed. Re-examine it and create a new set of decoders,
                    // then restart the decode loop. This is an advanced feature and it is not
                    // unreasonable to consider this "the end." As of v0.5.0, the only usage of this is
                    // for chained OGG physical streams.
                    unimplemented!();
                }
                Err(SymphoniaError::IoError(_)) => {
                    break;
                }
                Err(err) => {
                    // A unrecoverable error occured, halt decoding.
                    eprintln!("{:?}", err);
                    break;
                }
            };

            // Consume any new metadata that has been read since the last packet.
            while !format.metadata().is_latest() {
                // Pop the old head of the metadata queue.
                format.metadata().pop();

                // Consume the new metadata at the head of the metadata queue.
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != track_id {
                continue;
            }

            // Decode the packet into audio samples.
            match decoder.decode(&packet) {
                Ok(decoded) => {
                    if decoded.frames() > 0 {
                        let spec = *decoded.spec();
                        let mut samples: SampleBuffer<T> =
                            SampleBuffer::new(decoded.frames() as u64, spec);
                        samples.copy_interleaved_ref(decoded);
                        match channel_data.as_mut() {
                            Some((_, channels)) => {
                                for sample in samples.samples() {
                                    channels.push(*sample);
                                }
                            }
                            None => {
                                let channel_count = spec.channels.count();
                                let mut channels: Vec<T> = vec![];
                                for sample in samples.samples() {
                                    channels.push(*sample);
                                }
                                channel_data = Some((channel_count, channels));
                            }
                        }
                    }
                    // Consume the decoded audio samples (see below).
                }
                Err(SymphoniaError::IoError(_)) => {
                    // The packet failed to decode due to an IO error, skip the packet.
                    continue;
                }
                Err(SymphoniaError::DecodeError(_)) => {
                    // The packet failed to decode due to invalid data, skip the packet.
                    continue;
                }
                Err(err) => {
                    // An unrecoverable error occured, halt decoding.
                    panic!("{}", err);
                }
            }
        }

        let err_fn = |err| eprintln!("A playback error has occured! {}", err);

        let frame_index = Arc::new(RwLock::new(0));
        if let Some((_, channels)) = channel_data {
            let channels_len = channels.len();
            let stream = device.build_output_stream(
                config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    let offset = { *frame_index.read().unwrap() };
                    let mut i = 0;
                    if offset >= channels_len {
                        return;
                    }
                    for d in data {
                        if offset + i >= channels_len {
                            let mut offset = frame_index.write().unwrap();
                            *offset = channels_len;
                            let _ = stream_tx.send(StreamEvent::StreamEnded);
                            break;
                        }
                        *d = channels[offset + i];
                        i += 1;
                    }
                    {
                        let mut offset = frame_index.write().unwrap();
                        *offset += i;
                    }
                },
                err_fn,
            )?;
            stream.play()?;
            Ok(stream)
        } else {
            Err(DiziError::new(DiziErrorKind::NoDevice, "".to_string()))
        }
    }
}
