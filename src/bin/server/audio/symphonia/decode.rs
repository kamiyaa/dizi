use std::iter::Iterator;
use std::sync::{mpsc, Arc, RwLock};
use std::time::Duration;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatReader, Packet};

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{Stream, StreamConfig};

use dizi::error::{DiziError, DiziResult};
use symphonia::core::units::TimeBase;

use crate::audio::request::PlayerRequest;

use super::stream::StreamEvent;

pub struct PacketReader {
    format: Box<dyn FormatReader>,
    track_id: u32,
}

impl PacketReader {
    pub fn new(format: Box<dyn FormatReader>, track_id: u32) -> Self {
        Self { format, track_id }
    }
}

impl Iterator for PacketReader {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let packet = match self.format.next_packet() {
                Ok(p) => p,
                Err(_) => return None,
            };

            // Consume any new metadata that has been read since the last packet.
            while !self.format.metadata().is_latest() {
                // Pop the old head of the metadata queue.
                self.format.metadata().pop();

                // Consume the new metadata at the head of the metadata queue.
            }

            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id() != self.track_id {
                continue;
            }
            return Some(packet);
        }
    }
}

pub struct PacketDecoder {
    decoder: Box<dyn Decoder>,
}

impl PacketDecoder {
    pub fn new(decoder: Box<dyn Decoder>) -> Self {
        Self { decoder }
    }

    pub fn decode<T>(&mut self, packet: Packet) -> DiziResult<Vec<T>>
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
        // Decode the packet into audio samples.
        match self.decoder.decode(&packet) {
            Ok(decoded) => {
                if decoded.frames() > 0 {
                    let spec = *decoded.spec();
                    let mut samples: SampleBuffer<T> =
                        SampleBuffer::new(decoded.frames() as u64, spec);
                    samples.copy_interleaved_ref(decoded);

                    let sample_data: Vec<T> = samples.samples().to_vec();
                    Ok(sample_data)
                } else {
                    Ok(vec![])
                }
            }
            Err(SymphoniaError::IoError(_)) => Ok(vec![]),
            Err(SymphoniaError::DecodeError(_)) => Ok(vec![]),
            Err(err) => {
                tracing::error!("Unhandled symphonia error: {}", err);
                Err(DiziError::from(err))
            }
        }
    }
}

pub fn stream_loop<T>(
    stream_tx: mpsc::Sender<StreamEvent>,
    device: &cpal::Device,
    config: &StreamConfig,
    samples: Vec<T>,
    volume: f32,
    volume_change: fn(T, f32) -> T,
) -> DiziResult<(Stream, mpsc::Sender<PlayerRequest>)>
where
    T: symphonia::core::sample::Sample
        + cpal::Sample
        + cpal::SizedSample
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
    let err_fn = |err| eprintln!("A playback error has occured! {}", err);

    let (playback_loop_tx, playback_loop_rx) = mpsc::channel();

    let time_base = TimeBase {
        numer: 1,
        denom: config.sample_rate.0 * config.channels as u32,
    };

    let _ = stream_tx.send(StreamEvent::Progress(Duration::from_secs(0)));

    // if stream_tx is None, then we've already sent a StreamEnded message
    // and we don't need to send another one
    let mut stream_tx = Some(stream_tx);

    let samples_count = samples.len();

    // all vars that the stream will update while its streaming
    let frame_index = Arc::new(RwLock::new(0_usize));
    let volume = Arc::new(RwLock::new(volume));
    let playback_duration = Arc::new(RwLock::new(0));

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let process_message = |msg: PlayerRequest| match msg {
                PlayerRequest::SetVolume { volume: new_volume } => {
                    let mut current_volume = volume.write().unwrap();
                    *current_volume = new_volume;
                }
                PlayerRequest::FastForward { offset } => {
                    let mut sample_offset = frame_index.write().unwrap();
                    *sample_offset += time_base.denom as usize * offset.as_secs() as usize;
                    if *sample_offset >= samples_count {
                        *sample_offset = samples_count - time_base.denom as usize;
                    }
                }
                PlayerRequest::Rewind { offset } => {
                    let mut sample_offset = frame_index.write().unwrap();
                    if *sample_offset < time_base.denom as usize * offset.as_secs() as usize {
                        *sample_offset = 0;
                    } else {
                        *sample_offset -= time_base.denom as usize * offset.as_secs() as usize;
                    }
                }
                _ => {}
            };

            if let Ok(msg) = playback_loop_rx.try_recv() {
                process_message(msg);
            }

            // if sample_offsetcurrent_volume is greater than samples_count, then we've reached the end
            let sample_offset = { *frame_index.read().unwrap() };
            if sample_offset >= samples_count {
                if let Some(stream_tx) = stream_tx.take() {
                    let _ = stream_tx.send(StreamEvent::StreamEnded);
                }
                return;
            }

            let current_volume = { *volume.read().unwrap() };
            let mut i = 0;
            for d in data {
                if sample_offset + i >= samples_count {
                    let mut offset = frame_index.write().unwrap();
                    *offset = samples_count + 1;
                    break;
                }
                *d = volume_change(samples[sample_offset + i], current_volume);
                i += 1;
            }
            // new offset
            let new_sample_offset = {
                let mut sample_offset = frame_index.write().unwrap();
                *sample_offset += i;
                *sample_offset
            };
            // new duration
            let next_duration = time_base.calc_time(new_sample_offset as u64).seconds;
            let prev_duration = { *playback_duration.read().unwrap() };

            // update duration if seconds changed
            if prev_duration != next_duration {
                let new_duration = Duration::from_secs(next_duration);
                if let Some(stream_tx) = stream_tx.as_ref() {
                    let _ = stream_tx.send(StreamEvent::Progress(new_duration));
                }
                let mut duration = playback_duration.write().unwrap();
                *duration = new_duration.as_secs();
            }
        },
        err_fn,
        None,
    )?;
    stream.play()?;
    Ok((stream, playback_loop_tx))
}
