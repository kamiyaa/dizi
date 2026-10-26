use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU32, Ordering};
use std::sync::mpsc;
use std::time::Duration;

use cpal::traits::DeviceTrait;
use cpal::{FromSample, Sample, SizedSample, StreamConfig};
use symphonia::core::audio::conv::ConvertibleSample;

use dizi::error::{AppError, AppErrorCode, AppResult};

use super::{PlayerStreamEvent, StreamEvent};

/// Bounds for a sample type we can decode into, scale and hand to cpal
pub trait PlayableSample:
    ConvertibleSample + SizedSample + FromSample<f32> + Send + 'static
where
    f32: FromSample<Self>,
{
}

impl<T> PlayableSample for T
where
    T: ConvertibleSample + SizedSample + FromSample<f32> + Send + 'static,
    f32: FromSample<T>,
{
}

/// Playback settings shared between the server and the audio callback, so changing them
/// doesn't need a round trip through the stream thread.
#[derive(Debug)]
pub struct PlaybackControl {
    /// `f32` volume, stored as its bits
    volume: AtomicU32,
    /// Seek not yet applied by the audio callback, in milliseconds; negative rewinds
    pending_seek_ms: AtomicI64,
}

impl PlaybackControl {
    pub fn new(volume: f32) -> Self {
        Self {
            volume: AtomicU32::new(volume.to_bits()),
            pending_seek_ms: AtomicI64::new(0),
        }
    }

    pub fn volume(&self) -> f32 {
        f32::from_bits(self.volume.load(Ordering::Relaxed))
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume.store(volume.to_bits(), Ordering::Relaxed);
    }

    pub fn seek_by(&self, offset_ms: i64) {
        self.pending_seek_ms.fetch_add(offset_ms, Ordering::Relaxed);
    }

    /// Drops any seek not yet applied, e.g. when a new song starts
    pub fn clear_seek(&self) {
        self.pending_seek_ms.store(0, Ordering::Relaxed);
    }

    fn take_seek(&self) -> i64 {
        self.pending_seek_ms.swap(0, Ordering::Relaxed)
    }
}

/// Playback position within a fully decoded song, advanced by the audio callback
struct SampleCursor<T> {
    samples: Vec<T>,
    channels: usize,
    sample_rate: usize,
    /// Index of the next sample to play
    index: usize,
    elapsed_secs: u64,
    ended: bool,
}

impl<T: PlayableSample> SampleCursor<T>
where
    f32: FromSample<T>,
{
    fn new(samples: Vec<T>, config: &StreamConfig) -> AppResult<Self> {
        let channels = config.channels as usize;
        let sample_rate = config.sample_rate as usize;
        if channels == 0 || sample_rate == 0 {
            let error_msg = "Stream has no channels or a sample rate of 0";
            return Err(AppError::new(
                AppErrorCode::InvalidParameters,
                error_msg.to_string(),
            ));
        }
        Ok(Self {
            samples,
            channels,
            sample_rate,
            index: 0,
            elapsed_secs: 0,
            ended: false,
        })
    }

    fn samples_per_sec(&self) -> usize {
        self.sample_rate * self.channels
    }

    fn seek_by(&mut self, offset_ms: i64) {
        // move by whole frames, so samples stay aligned with their channel
        let frames = offset_ms.unsigned_abs() as usize * self.sample_rate / 1000;
        let delta = frames * self.channels;
        if offset_ms >= 0 {
            // stop a second short of the end, so the song doesn't immediately finish;
            // but never move backwards
            let limit = self.samples.len().saturating_sub(self.samples_per_sec());
            self.index = (self.index + delta).min(limit.max(self.index));
        } else {
            self.index = self.index.saturating_sub(delta);
        }
    }

    /// Fills `data` with the next samples at the current volume, applying any pending seek.
    /// Returns an event to report, if any.
    fn fill(&mut self, data: &mut [T], control: &PlaybackControl) -> Option<StreamEvent> {
        let seek_ms = control.take_seek();
        if seek_ms != 0 {
            self.seek_by(seek_ms);
        }

        let volume = control.volume();
        let remaining = self.samples.get(self.index..).unwrap_or_default();
        let count = remaining.len().min(data.len());
        for (out, sample) in data.iter_mut().zip(&remaining[..count]) {
            *out = scale_volume(*sample, volume);
        }
        // past the end of the song, play silence rather than whatever is left in the buffer
        data[count..].fill(T::EQUILIBRIUM);
        self.index += count;

        if self.index >= self.samples.len() {
            if self.ended {
                return None;
            }
            self.ended = true;
            return Some(StreamEvent::StreamEnded);
        }

        let elapsed_secs = (self.index / self.samples_per_sec()) as u64;
        if elapsed_secs == self.elapsed_secs {
            return None;
        }
        self.elapsed_secs = elapsed_secs;
        Some(StreamEvent::Progress(Duration::from_secs(elapsed_secs)))
    }
}

/// Scales through `f32`, so unsigned formats scale around their midpoint rather than toward 0
fn scale_volume<T: PlayableSample>(sample: T, volume: f32) -> T
where
    f32: FromSample<T>,
{
    <T as Sample>::from_sample(<f32 as Sample>::from_sample(sample) * volume)
}

/// Builds a (paused) output stream that plays `samples`
pub fn build_stream<T: PlayableSample>(
    stream_tx: mpsc::Sender<PlayerStreamEvent>,
    device: &cpal::Device,
    config: &StreamConfig,
    samples: Vec<T>,
    control: Arc<PlaybackControl>,
) -> AppResult<cpal::Stream>
where
    f32: FromSample<T>,
{
    let mut cursor = SampleCursor::new(samples, config)?;

    let _ = stream_tx.send(PlayerStreamEvent::Stream(StreamEvent::Progress(
        Duration::ZERO,
    )));

    let stream = device.build_output_stream(
        *config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            if let Some(event) = cursor.fill(data, &control) {
                let _ = stream_tx.send(PlayerStreamEvent::Stream(event));
            }
        },
        |err| tracing::error!(?err, "A playback error has occured!"),
        None,
    )?;
    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(channels: u16, sample_rate: u32) -> StreamConfig {
        StreamConfig {
            channels,
            sample_rate,
            buffer_size: cpal::BufferSize::Default,
        }
    }

    #[test]
    fn end_of_song_plays_silence_and_reports_once() {
        let control = PlaybackControl::new(1.0);
        let mut cursor = SampleCursor::new(vec![200u8; 3], &config(1, 10)).unwrap();
        let mut data = [0u8; 5];

        let event = cursor.fill(&mut data, &control);
        assert_eq!(data, [200, 200, 200, 128, 128]);
        assert_eq!(event, Some(StreamEvent::StreamEnded));
        assert_eq!(cursor.fill(&mut data, &control), None);
        assert_eq!(data, [128; 5]);
    }

    #[test]
    fn fast_forward_on_short_song_does_not_underflow() {
        let control = PlaybackControl::new(1.0);
        // half a second of audio
        let mut cursor = SampleCursor::new(vec![0.5f32; 10], &config(2, 10)).unwrap();
        control.seek_by(5000);
        let mut data = [0.0f32; 2];
        cursor.fill(&mut data, &control);
        assert_eq!(cursor.index, 2);
    }

    #[test]
    fn seeking_keeps_channels_aligned() {
        let control = PlaybackControl::new(1.0);
        let mut cursor = SampleCursor::new(vec![0i16; 2 * 1000], &config(2, 100)).unwrap();
        control.seek_by(1015);
        cursor.fill(&mut [], &control);
        assert_eq!(cursor.index, 2 * 101);
        control.seek_by(-500);
        cursor.fill(&mut [], &control);
        assert_eq!(cursor.index, 2 * 51);
    }

    #[test]
    fn volume_scales_unsigned_around_midpoint() {
        assert_eq!(scale_volume(128u8, 0.5), 128);
        assert_eq!(scale_volume(255u8, 0.0), 128);
        assert_eq!(scale_volume(-1000i16, 0.5), -500);
    }

    #[test]
    fn progress_is_reported_once_per_second() {
        let control = PlaybackControl::new(1.0);
        let mut cursor = SampleCursor::new(vec![0.0f32; 30], &config(1, 10)).unwrap();
        let mut data = [0.0f32; 5];
        assert_eq!(cursor.fill(&mut data, &control), None);
        assert_eq!(
            cursor.fill(&mut data, &control),
            Some(StreamEvent::Progress(Duration::from_secs(1)))
        );
        assert_eq!(cursor.fill(&mut data, &control), None);
    }
}
