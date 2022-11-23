use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use log::{debug, log_enabled, Level};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::Stream;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::audio::request::PlayerRequest;
use crate::events::{ServerEvent, ServerEventSender};

use super::decode::{stream_loop, PacketDecoder, PacketReader};
#[derive(Clone, Copy, Debug)]

pub enum StreamEvent {
    Progress(Duration),
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

    pub fn pause(&mut self) -> DiziResult {
        if let Some(state) = self.state.as_ref() {
            state.stream.pause()?;
        }
        Ok(())
    }
    pub fn resume(&mut self) -> DiziResult {
        if let Some(state) = self.state.as_ref() {
            state.stream.play()?;
        }
        Ok(())
    }
    pub fn stop(&mut self) -> DiziResult {
        self.state.take();
        Ok(())
    }
    pub fn fast_forward(&mut self, offset: Duration) -> DiziResult {
        if let Some(state) = self.state.as_ref() {
            state
                .playback_loop_tx
                .send(PlayerRequest::FastForward { offset })?;
        }
        Ok(())
    }
    pub fn rewind(&mut self, offset: Duration) -> DiziResult {
        if let Some(state) = self.state.as_ref() {
            state
                .playback_loop_tx
                .send(PlayerRequest::Rewind { offset })?;
        }
        Ok(())
    }

    pub fn set_volume(&mut self, volume: f32) {
        if let Some(state) = self.state.as_ref() {
            let _ = state
                .playback_loop_tx
                .send(PlayerRequest::SetVolume { volume });
        }
    }

    pub fn listen_for_events(&mut self) -> DiziResult {
        while let Ok(msg) = self.event_poller.next() {
            match msg {
                PlayerStreamEvent::Player(req) => self.process_player_req(req)?,
                PlayerStreamEvent::Stream(event) => self.process_stream_event(event)?,
            }
        }
        Ok(())
    }

    fn process_player_req(&mut self, req: PlayerRequest) -> DiziResult {
        match req {
            PlayerRequest::Play { song, volume } => {
                let stream_res = self.play(song.file_path(), volume);
                match stream_res {
                    Ok(stream_res) => {
                        let (stream, playback_loop_tx) = stream_res;
                        self.state = Some(PlayerStreamState {
                            stream,
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
            PlayerRequest::SetVolume { volume } => {
                self.set_volume(volume);
                self.event_poller.player_res().send(Ok(()))?;
            }
            PlayerRequest::FastForward { offset } => {
                self.fast_forward(offset)?;
            }
            PlayerRequest::Rewind { offset } => {
                self.rewind(offset)?;
            }
        }
        Ok(())
    }

    fn process_stream_event(&mut self, event: StreamEvent) -> DiziResult {
        match event {
            StreamEvent::StreamEnded => {
                self.stop()?;
                self.event_tx.send(ServerEvent::PlayerDone)?;
            }
            StreamEvent::Progress(duration) => {
                self.event_tx
                    .send(ServerEvent::PlayerProgressUpdate(duration))?;
            }
        }
        Ok(())
    }

    pub fn play(
        &self,
        path: &Path,
        volume: f32,
    ) -> DiziResult<(Stream, mpsc::Sender<PlayerRequest>)> {
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
            None => Err(DiziError::new(DiziErrorKind::NoDevice, "".to_string())),
            Some(track) => {
                // Store the track identifier, it will be used to filter packets.
                let track_id = track.id;

                if log_enabled!(Level::Debug) {
                    debug!("track: {:#?}", track);
                }

                // Use the default options for the decoder.
                let dec_opts: DecoderOptions = Default::default();

                // Create a decoder for the track.
                let decoder =
                    symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)?;

                let config = self.device.default_output_config().unwrap();

                let audio_config = cpal::StreamConfig {
                    channels: track
                        .codec_params
                        .channels
                        .map(|c| c.count() as u16)
                        .unwrap_or(2u16),
                    sample_rate: cpal::SampleRate(
                        track
                            .codec_params
                            .sample_rate
                            .unwrap_or_else(|| config.sample_rate().0),
                    ),
                    buffer_size: cpal::BufferSize::Default,
                };

                if log_enabled!(Level::Debug) {
                    debug!("audio_config: {:#?}", audio_config);
                }

                let stream_tx = self.event_poller.stream_tx.clone();

                match config.sample_format() {
                    cpal::SampleFormat::F32 => {
                        let packet_reader = PacketReader::new(probed.format, track_id);
                        let mut packet_decoder = PacketDecoder::new(decoder);
                        let mut samples = Vec::new();
                        for packet in packet_reader {
                            let packet_sample = packet_decoder.decode::<f32>(packet)?;
                            samples.extend(packet_sample);
                        }
                        let res = stream_loop::<f32>(
                            stream_tx,
                            &self.device,
                            &audio_config,
                            samples,
                            volume,
                            |packet, volume| packet * volume,
                        )?;
                        Ok(res)
                    }
                    cpal::SampleFormat::I16 => {
                        let packet_reader = PacketReader::new(probed.format, track_id);
                        let mut packet_decoder = PacketDecoder::new(decoder);
                        let mut samples = Vec::new();
                        for packet in packet_reader {
                            let packet_sample = packet_decoder.decode::<i16>(packet)?;
                            samples.extend(packet_sample);
                        }
                        let res = stream_loop::<i16>(
                            stream_tx,
                            &self.device,
                            &audio_config,
                            samples,
                            volume,
                            |packet, volume| ((packet as f32) * volume) as i16,
                        )?;
                        Ok(res)
                    }
                    cpal::SampleFormat::U16 => {
                        let packet_reader = PacketReader::new(probed.format, track_id);
                        let mut packet_decoder = PacketDecoder::new(decoder);
                        let mut samples = Vec::new();
                        for packet in packet_reader {
                            let packet_sample = packet_decoder.decode::<u16>(packet)?;
                            samples.extend(packet_sample);
                        }
                        let res = stream_loop::<u16>(
                            stream_tx,
                            &self.device,
                            &audio_config,
                            samples,
                            volume,
                            |packet, volume| ((packet as f32) * volume) as u16,
                        )?;
                        Ok(res)
                    }
                }
            }
        }
    }
}
