use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dizi::song::DiziAudioFile;
use symphonia::core::codecs::DecoderOptions;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::Stream;

use dizi::error::{DiziError, DiziErrorKind, DiziResult};

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
pub struct PlayerStreamEventListener {
    pub stream_tx: mpsc::Sender<StreamEvent>,
    pub player_res_tx: mpsc::Sender<DiziResult>,
    pub _event_tx: mpsc::Sender<PlayerStreamEvent>,
    pub event_rx: mpsc::Receiver<PlayerStreamEvent>,
}

impl PlayerStreamEventListener {
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
            _event_tx: event_tx,
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
    event_poller: PlayerStreamEventListener,
    device: cpal::Device,
    stream_config: cpal::SupportedStreamConfig,
    state: Option<PlayerStreamState>,
}

impl PlayerStream {
    pub fn new(
        event_tx: ServerEventSender,
        player_res_tx: mpsc::Sender<DiziResult>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
        device: cpal::Device,
    ) -> DiziResult<Self> {
        let event_poller = PlayerStreamEventListener::new(player_res_tx, player_req_rx);

        let stream_config = device.default_output_config().map_err(|err| {
            let error_msg = "Failed to get default output config";
            tracing::error!("{error_msg}: {err}");
            DiziError::new(DiziErrorKind::Symphonia, error_msg.to_string())
        })?;

        tracing::debug!("stream config: {:#?}", stream_config);

        Ok(Self {
            event_tx,
            event_poller,
            device,
            stream_config,
            state: None,
        })
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
                let stream_res = self.play(song, volume);
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
        audio_file: DiziAudioFile,
        volume: f32,
    ) -> DiziResult<(Stream, mpsc::Sender<PlayerRequest>)> {
        let track_id = audio_file.audio_metadata.track_id;

        let probe_result = audio_file.file.get_probe_result()?;

        let codec_params = probe_result
            .format
            .default_track()
            .map(|t| &t.codec_params)
            .ok_or_else(|| {
                let error_msg = "Failed to get default track codec_params";
                tracing::error!("{error_msg}");
                DiziError::new(DiziErrorKind::Symphonia, error_msg.to_string())
            })?;

        // Use the default options for the decoder.
        let dec_opts: DecoderOptions = Default::default();

        // Create a decoder for the track.
        let decoder = symphonia::default::get_codecs().make(&codec_params, &dec_opts)?;

        let audio_config = cpal::StreamConfig {
            channels: audio_file
                .audio_metadata
                .channels
                .map(|c| c as u16)
                .unwrap_or_else(|| self.stream_config.channels()),
            sample_rate: cpal::SampleRate(
                audio_file
                    .audio_metadata
                    .sample_rate
                    .unwrap_or_else(|| self.stream_config.sample_rate().0),
            ),
            buffer_size: cpal::BufferSize::Default,
        };

        tracing::debug!("audio_config: {:#?}", audio_config);

        let stream_tx = self.event_poller.stream_tx.clone();

        let packet_reader = PacketReader::new(probe_result.format, track_id);
        let mut packet_decoder = PacketDecoder::new(decoder);

        match self.stream_config.sample_format() {
            cpal::SampleFormat::U8 => {
                let mut samples = Vec::new();
                for packet in packet_reader {
                    let packet_sample = packet_decoder.decode::<u8>(packet)?;
                    samples.extend(packet_sample);
                }
                let res = stream_loop::<u8>(
                    stream_tx,
                    &self.device,
                    &audio_config,
                    samples,
                    volume,
                    |packet, volume| ((packet as f32) * volume) as u8,
                )?;
                Ok(res)
            }
            cpal::SampleFormat::U16 => {
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
            cpal::SampleFormat::U32 => {
                let mut samples = Vec::new();
                for packet in packet_reader {
                    let packet_sample = packet_decoder.decode::<u32>(packet)?;
                    samples.extend(packet_sample);
                }
                let res = stream_loop::<u32>(
                    stream_tx,
                    &self.device,
                    &audio_config,
                    samples,
                    volume,
                    |packet, volume| ((packet as f32) * volume) as u32,
                )?;
                Ok(res)
            }
            cpal::SampleFormat::I8 => {
                let mut samples = Vec::new();
                for packet in packet_reader {
                    let packet_sample = packet_decoder.decode::<i8>(packet)?;
                    samples.extend(packet_sample);
                }
                let res = stream_loop::<i8>(
                    stream_tx,
                    &self.device,
                    &audio_config,
                    samples,
                    volume,
                    |packet, volume| ((packet as f32) * volume) as i8,
                )?;
                Ok(res)
            }
            cpal::SampleFormat::I16 => {
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
            cpal::SampleFormat::I32 => {
                let mut samples = Vec::new();
                for packet in packet_reader {
                    let packet_sample = packet_decoder.decode::<i32>(packet)?;
                    samples.extend(packet_sample);
                }
                let res = stream_loop::<i32>(
                    stream_tx,
                    &self.device,
                    &audio_config,
                    samples,
                    volume,
                    |packet, volume| ((packet as f32) * volume) as i32,
                )?;
                Ok(res)
            }
            cpal::SampleFormat::F32 => {
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
            _ => {
                let mut samples = Vec::new();
                for packet in packet_reader {
                    let packet_sample = packet_decoder.decode::<f64>(packet)?;
                    samples.extend(packet_sample);
                }
                let res = stream_loop::<f64>(
                    stream_tx,
                    &self.device,
                    &audio_config,
                    samples,
                    volume,
                    |packet, volume| (packet * volume as f64) as f64,
                )?;
                Ok(res)
            }
        }
    }
}
