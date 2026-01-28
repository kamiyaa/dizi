use std::sync::mpsc;
use std::time::Duration;

use cpal::traits::DeviceTrait;
use cpal::Stream;
use cpal::StreamConfig;
use symphonia::core::units::TimeBase;

use dizi::error::DiziResult;

use crate::audio::request::PlayerRequest;

use super::StreamEvent;

/// Stream state
pub struct PlayerStreamState {
    pub stream: Stream,
    pub playback_loop_tx: mpsc::Sender<PlayerRequest>,
}

impl PlayerStreamState {
    pub fn build<T>(
        stream_tx: mpsc::Sender<StreamEvent>,
        device: &cpal::Device,
        config: &StreamConfig,
        samples: Vec<T>,
        volume: f32,
        volume_change: fn(T, f32) -> T,
    ) -> DiziResult<PlayerStreamState>
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
        build_stream_state(stream_tx, device, config, samples, volume, volume_change)
    }
}

fn build_stream_state<T>(
    stream_tx: mpsc::Sender<StreamEvent>,
    device: &cpal::Device,
    config: &StreamConfig,
    samples: Vec<T>,
    volume: f32,
    volume_change: fn(T, f32) -> T,
) -> DiziResult<PlayerStreamState>
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
    let err_fn = |err| {
        tracing::error!(?err, "A playback error has occured!");
    };

    let time_base = TimeBase {
        numer: 1,
        denom: config.sample_rate * config.channels as u32,
    };

    let samples_count = samples.len();

    // all vars that the stream will update while its streaming
    let mut volume = volume;
    // The currrent frame_index, "where the cursor is within the file"
    let mut frame_index: usize = 0;
    let mut playback_duration = 0;

    // initial event
    let _ = stream_tx.send(StreamEvent::Progress(Duration::from_secs(0)));

    // if stream_tx is None, then we've already sent a StreamEnded message
    // and we don't need to send another one
    let mut stream_tx = Some(stream_tx);

    let (playback_loop_tx, playback_loop_rx) = mpsc::channel();

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // Process any user requests
            if let Ok(msg) = playback_loop_rx.try_recv() {
                match msg {
                    PlayerRequest::SetVolume { volume: new_volume } => {
                        volume = new_volume;
                    }
                    PlayerRequest::FastForward { offset } => {
                        frame_index += time_base.denom as usize * offset.as_secs() as usize;
                        if frame_index >= samples_count {
                            frame_index = samples_count - time_base.denom as usize;
                        }
                    }
                    PlayerRequest::Rewind { offset } => {
                        if frame_index < time_base.denom as usize * offset.as_secs() as usize {
                            frame_index = 0;
                        } else {
                            frame_index -= time_base.denom as usize * offset.as_secs() as usize;
                        }
                    }
                    _ => {}
                }
            }

            // if frame_index is greater than samples_count, then we've reached the end
            if frame_index >= samples_count {
                if let Some(stream_tx) = stream_tx.take() {
                    let _ = stream_tx.send(StreamEvent::StreamEnded);
                }
                return;
            }

            let current_volume = volume;
            let mut i = 0;
            for d in data.iter_mut() {
                if frame_index + i >= samples_count {
                    frame_index = samples_count + 1;
                    break;
                }
                *d = volume_change(samples[frame_index + i], current_volume);
                i += 1;
            }
            // new offset
            let new_sample_offset = {
                frame_index += i;
                frame_index
            };
            // new duration
            let next_duration = time_base.calc_time(new_sample_offset as u64).seconds;
            let prev_duration = playback_duration;

            // only update duration if seconds changed
            if prev_duration != next_duration {
                let new_duration = Duration::from_secs(next_duration);
                if let Some(stream_tx) = stream_tx.as_ref() {
                    let _ = stream_tx.send(StreamEvent::Progress(new_duration));
                }
                playback_duration = new_duration.as_secs();
            }
        },
        err_fn,
        None,
    )?;
    let state = PlayerStreamState {
        stream,
        playback_loop_tx,
    };
    Ok(state)
}
