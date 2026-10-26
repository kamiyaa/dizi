use std::sync::{Arc, mpsc};

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{FromSample, SampleFormat};
use symphonia::core::codecs::CodecParameters;
use symphonia::core::codecs::audio::AudioDecoderOptions;

use dizi::error::{AppError, AppErrorCode, AppResult};
use dizi::song::DiziAudioFile;

use crate::audio::request::PlayerRequest;
use crate::audio::symphonia::stream::{
    PlayableSample, PlaybackControl, PlayerStreamEvent, StreamEvent, build_stream,
};
use crate::events::{AppEventSender, ServerEvent};

use super::super::decode::{PacketDecoder, PacketReader, decode_all};

/// Owns the audio output stream. Runs on its own thread, since `cpal::Stream` can't be sent
/// between threads on every platform.
pub struct PlayerStream {
    event_tx: AppEventSender,
    /// Handed to each output stream, to report progress back to this thread
    stream_event_tx: mpsc::Sender<PlayerStreamEvent>,
    stream_event_rx: mpsc::Receiver<PlayerStreamEvent>,
    player_res_tx: mpsc::Sender<AppResult>,
    control: Arc<PlaybackControl>,
    device: cpal::Device,
    stream_config: cpal::SupportedStreamConfig,
    stream: Option<cpal::Stream>,
}

impl PlayerStream {
    pub fn new(
        event_tx: AppEventSender,
        stream_event_tx: mpsc::Sender<PlayerStreamEvent>,
        stream_event_rx: mpsc::Receiver<PlayerStreamEvent>,
        player_res_tx: mpsc::Sender<AppResult>,
        control: Arc<PlaybackControl>,
        device: cpal::Device,
    ) -> AppResult<Self> {
        let stream_config = device.default_output_config().map_err(|err| {
            let error_msg = "Failed to get default output config";
            tracing::error!(?err, "{error_msg}");
            AppError::new(AppErrorCode::Symphonia, error_msg.to_string())
        })?;

        tracing::debug!(?stream_config, "stream config");

        Ok(Self {
            event_tx,
            stream_event_tx,
            stream_event_rx,
            player_res_tx,
            control,
            device,
            stream_config,
            stream: None,
        })
    }

    pub fn listen_for_events(&mut self) -> AppResult {
        while let Ok(msg) = self.stream_event_rx.recv() {
            match msg {
                PlayerStreamEvent::Player(req) => {
                    let res = self.process_player_req(req);
                    self.player_res_tx.send(res)?;
                }
                PlayerStreamEvent::Stream(event) => self.process_stream_event(event)?,
            }
        }
        Ok(())
    }

    fn process_player_req(&mut self, req: PlayerRequest) -> AppResult {
        match req {
            PlayerRequest::Play { song } => {
                self.control.clear_seek();
                let stream = self.build_stream(&song)?;
                stream.play()?;
                self.stream = Some(stream);
            }
            PlayerRequest::Pause => {
                if let Some(stream) = self.stream.as_ref() {
                    stream.pause()?;
                }
            }
            PlayerRequest::Resume => {
                if let Some(stream) = self.stream.as_ref() {
                    stream.play()?;
                }
            }
            PlayerRequest::Stop => {
                self.stream = None;
            }
        }
        Ok(())
    }

    fn process_stream_event(&mut self, event: StreamEvent) -> AppResult {
        match event {
            StreamEvent::StreamEnded => {
                self.stream = None;
                self.event_tx.send(ServerEvent::PlayerDone.into())?;
            }
            StreamEvent::Progress(duration) => {
                self.event_tx
                    .send(ServerEvent::PlayerProgressUpdate(duration).into())?;
            }
        }
        Ok(())
    }

    fn build_stream(&self, audio_file: &DiziAudioFile) -> AppResult<cpal::Stream> {
        let track_id = audio_file.audio_metadata.track_id;

        let format_reader = audio_file.file.get_probe_result()?;

        let codec_params = format_reader
            .tracks()
            .first()
            .ok_or_else(|| {
                let error_msg = "No tracks found";
                tracing::error!(?audio_file.file, "{error_msg}");
                AppError::new(AppErrorCode::Symphonia, error_msg.to_string())
            })?
            .codec_params
            .as_ref()
            .ok_or_else(|| {
                let error_msg = "Failed to get default track codec_params";
                tracing::error!("{error_msg}");
                AppError::new(AppErrorCode::Symphonia, error_msg.to_string())
            })?;

        let audio_codec_params = match codec_params {
            CodecParameters::Audio(params) => params,
            _ => {
                let error_msg = "Codec not audio";
                tracing::error!("{error_msg}");
                let err = AppError::new(AppErrorCode::Parse, error_msg.to_string());
                return Err(err);
            }
        };

        // Use the default options for the decoder.
        let dec_opts: AudioDecoderOptions = Default::default();

        // Create a decoder for the track.
        let decoder =
            symphonia::default::get_codecs().make_audio_decoder(audio_codec_params, &dec_opts)?;

        let packet_reader = PacketReader::new(format_reader, track_id);
        let packet_decoder = PacketDecoder::new(decoder);

        // Fallback used only if decoding produced no packets at all (e.g. an
        // empty file), in which case there's no audio to misconfigure anyway.
        let fallback_audio_config = cpal::StreamConfig {
            channels: audio_file
                .audio_metadata
                .channels
                .map(|c| c as u16)
                .unwrap_or_else(|| self.stream_config.channels()),
            sample_rate: audio_file
                .audio_metadata
                .sample_rate
                .unwrap_or_else(|| self.stream_config.sample_rate()),
            buffer_size: cpal::BufferSize::Default,
        };

        match self.stream_config.sample_format() {
            SampleFormat::U8 => {
                self.decode_and_build::<u8>(packet_reader, packet_decoder, fallback_audio_config)
            }
            SampleFormat::U16 => {
                self.decode_and_build::<u16>(packet_reader, packet_decoder, fallback_audio_config)
            }
            SampleFormat::U32 => {
                self.decode_and_build::<u32>(packet_reader, packet_decoder, fallback_audio_config)
            }
            SampleFormat::I8 => {
                self.decode_and_build::<i8>(packet_reader, packet_decoder, fallback_audio_config)
            }
            SampleFormat::I16 => {
                self.decode_and_build::<i16>(packet_reader, packet_decoder, fallback_audio_config)
            }
            SampleFormat::I32 => {
                self.decode_and_build::<i32>(packet_reader, packet_decoder, fallback_audio_config)
            }
            SampleFormat::F32 => {
                self.decode_and_build::<f32>(packet_reader, packet_decoder, fallback_audio_config)
            }
            _ => self.decode_and_build::<f64>(packet_reader, packet_decoder, fallback_audio_config),
        }
    }

    fn decode_and_build<T: PlayableSample>(
        &self,
        packet_reader: PacketReader,
        packet_decoder: PacketDecoder,
        fallback_audio_config: cpal::StreamConfig,
    ) -> AppResult<cpal::Stream>
    where
        f32: FromSample<T>,
    {
        let (samples, spec) = decode_all::<T>(packet_reader, packet_decoder)?;
        let audio_config = match spec {
            Some(spec) => cpal::StreamConfig {
                channels: spec.channels as u16,
                sample_rate: spec.sample_rate,
                buffer_size: cpal::BufferSize::Default,
            },
            None => fallback_audio_config,
        };
        build_stream(
            self.stream_event_tx.clone(),
            &self.device,
            &audio_config,
            samples,
            self.control.clone(),
        )
    }
}
