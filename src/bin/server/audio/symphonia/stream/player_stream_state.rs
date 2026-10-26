use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;

use cpal::traits::DeviceTrait;
use cpal::{FromSample, Sample, SizedSample, StreamConfig};
use symphonia::core::audio::conv::ConvertibleSample;
use symphonia::core::units::Time;

use dizi::error::{AppError, AppErrorCode, AppResult};

use super::super::decode::TrackDecoder;
use super::{PlayerStreamEvent, StreamEvent};

/// How many decoded packets to keep queued ahead of playback; roughly 0.5 to 1.5 seconds,
/// depending on the codec's packet size
const QUEUED_CHUNKS: usize = 16;

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

/// Playback settings shared between the server and the audio stream, so changing them
/// doesn't need a round trip through the stream thread.
#[derive(Debug)]
pub struct PlaybackControl {
    /// `f32` volume, stored as its bits
    volume: AtomicU32,
    /// Seek not yet applied by the decode thread, in milliseconds; negative rewinds
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

/// What to play: a decoder positioned after its first packet, which was already decoded to
/// learn the stream's real channel count and sample rate
pub struct StreamSource<T> {
    pub decoder: TrackDecoder,
    pub first_samples: Vec<T>,
    /// From the file's headers, if known; used to keep fast-forward short of the end
    pub total_duration: Option<Duration>,
}

/// Decoded samples starting at `start_frame`
struct Chunk<T> {
    /// Chunks from before the latest seek have an older generation, and are skipped
    generation: u64,
    start_frame: u64,
    samples: Vec<T>,
}

/// State shared by one stream's decode thread and audio callback
#[derive(Debug, Default)]
struct StreamPosition {
    /// Bumped by the decode thread on every seek
    generation: AtomicU64,
    /// Frame the audio callback has played up to
    played_frames: AtomicU64,
}

/// The audio callback's side of the stream: plays queued chunks
struct ChunkCursor<T> {
    chunk_rx: Receiver<Chunk<T>>,
    /// Hands played buffers back to the decode thread, so neither side allocates per packet
    recycle_tx: SyncSender<Vec<T>>,
    position: Arc<StreamPosition>,
    channels: usize,
    sample_rate: u64,
    current: Option<Chunk<T>>,
    /// Index of the next sample to play in `current`
    offset: usize,
    generation: u64,
    elapsed_secs: u64,
    ended: bool,
}

impl<T: PlayableSample> ChunkCursor<T>
where
    f32: FromSample<T>,
{
    fn recycle(&self, samples: Vec<T>) {
        let _ = self.recycle_tx.try_send(samples);
    }

    /// Makes `current` a chunk with samples left to play. Returns `false` if there is none
    /// queued, and records whether that's because the song ended.
    fn next_chunk(&mut self, finished: &mut bool) -> bool {
        loop {
            if let Some(chunk) = self.current.as_ref()
                && chunk.generation >= self.generation
                && self.offset < chunk.samples.len()
            {
                return true;
            }
            if let Some(chunk) = self.current.take() {
                self.recycle(chunk.samples);
            }
            match self.chunk_rx.try_recv() {
                Ok(chunk) if chunk.generation < self.generation => self.recycle(chunk.samples),
                Ok(chunk) => {
                    self.generation = chunk.generation;
                    self.current = Some(chunk);
                    self.offset = 0;
                }
                // decoding fell behind
                Err(TryRecvError::Empty) => return false,
                Err(TryRecvError::Disconnected) => {
                    *finished = true;
                    return false;
                }
            }
        }
    }

    /// Fills `data` with the next samples at `volume`. Returns an event to report, if any.
    fn fill(&mut self, data: &mut [T], volume: f32) -> Option<StreamEvent> {
        self.generation = self
            .generation
            .max(self.position.generation.load(Ordering::Acquire));

        let mut finished = false;
        let mut written = 0;
        while written < data.len() && self.next_chunk(&mut finished) {
            let Some(chunk) = self.current.as_ref() else {
                break;
            };
            let remaining = &chunk.samples[self.offset..];
            let count = remaining.len().min(data.len() - written);
            for (out, sample) in data[written..written + count].iter_mut().zip(remaining) {
                *out = scale_volume(*sample, volume);
            }
            self.offset += count;
            written += count;
        }
        // with nothing to play, play silence rather than whatever is left in the buffer
        data[written..].fill(T::EQUILIBRIUM);

        if finished {
            if self.ended {
                return None;
            }
            self.ended = true;
            return Some(StreamEvent::StreamEnded);
        }

        let chunk = self.current.as_ref()?;
        let played_frames = chunk.start_frame + (self.offset / self.channels) as u64;
        self.position
            .played_frames
            .store(played_frames, Ordering::Relaxed);

        let elapsed_secs = played_frames / self.sample_rate;
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

/// Where a seek by `offset_ms` from `position_ms` should land. Forward seeks stop a second
/// short of the end (when the length is known), so the song doesn't immediately finish, but
/// never move backwards.
fn seek_target_ms(position_ms: u64, offset_ms: i64, total_ms: Option<u64>) -> u64 {
    if offset_ms < 0 {
        return position_ms.saturating_sub(offset_ms.unsigned_abs());
    }
    let target = position_ms.saturating_add(offset_ms as u64);
    match total_ms {
        Some(total_ms) => target.min(total_ms.saturating_sub(1000).max(position_ms)),
        None => target,
    }
}

/// Decodes the rest of the song into `chunk_tx`, applying seeks as they come in. Returns
/// when the song ends or the stream is dropped.
fn decode_loop<T: PlayableSample>(
    source: StreamSource<T>,
    sample_rate: u32,
    channels: usize,
    chunk_tx: SyncSender<Chunk<T>>,
    recycle_rx: Receiver<Vec<T>>,
    position: Arc<StreamPosition>,
    control: Arc<PlaybackControl>,
) where
    f32: FromSample<T>,
{
    let StreamSource {
        mut decoder,
        first_samples,
        total_duration,
    } = source;
    let total_ms = total_duration.map(|d| d.as_millis() as u64);

    let mut generation = 0;
    let mut next_frame = 0;
    let mut samples = first_samples;
    loop {
        let frames = (samples.len() / channels) as u64;
        let chunk = Chunk {
            generation,
            start_frame: next_frame,
            samples,
        };
        // blocks while the queue is full; fails once the stream is dropped
        if chunk_tx.send(chunk).is_err() {
            return;
        }
        next_frame += frames;

        let seek_ms = control.take_seek();
        if seek_ms != 0 {
            let played_frames = position.played_frames.load(Ordering::Relaxed);
            let position_ms = played_frames * 1000 / sample_rate as u64;
            let target_ms = seek_target_ms(position_ms, seek_ms, total_ms);
            match decoder.seek(Time::from_millis_u64(target_ms), sample_rate) {
                Ok(frame) => {
                    generation += 1;
                    position.generation.store(generation, Ordering::Release);
                    next_frame = frame;
                }
                Err(err) => tracing::debug!(?err, target_ms, "Failed to seek"),
            }
        }

        let mut buf = recycle_rx.try_recv().unwrap_or_default();
        match decoder.decode_next(&mut buf) {
            Ok(Some(_)) => samples = buf,
            // dropping `chunk_tx` tells the audio callback the song is over
            Ok(None) => return,
            Err(err) => {
                tracing::error!(?err, "Failed to decode, ending song");
                return;
            }
        }
    }
}

/// Builds a (paused) output stream for `source`, decoding the rest of it on a background
/// thread as it plays. Events are tagged with `stream_id`.
pub fn build_stream<T: PlayableSample>(
    stream_tx: mpsc::Sender<PlayerStreamEvent>,
    stream_id: u64,
    device: &cpal::Device,
    config: &StreamConfig,
    source: StreamSource<T>,
    control: Arc<PlaybackControl>,
) -> AppResult<cpal::Stream>
where
    f32: FromSample<T>,
{
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate;
    if channels == 0 || sample_rate == 0 {
        let error_msg = "Stream has no channels or a sample rate of 0";
        return Err(AppError::new(
            AppErrorCode::InvalidParameters,
            error_msg.to_string(),
        ));
    }

    let (chunk_tx, chunk_rx) = mpsc::sync_channel(QUEUED_CHUNKS);
    let (recycle_tx, recycle_rx) = mpsc::sync_channel(QUEUED_CHUNKS + 2);
    let position = Arc::new(StreamPosition::default());

    let mut cursor = ChunkCursor {
        chunk_rx,
        recycle_tx,
        position: position.clone(),
        channels,
        sample_rate: sample_rate as u64,
        current: None,
        offset: 0,
        generation: 0,
        elapsed_secs: 0,
        ended: false,
    };

    let _ = stream_tx.send(PlayerStreamEvent::Stream {
        id: stream_id,
        event: StreamEvent::Progress(Duration::ZERO),
    });

    let callback_control = control.clone();
    let stream = device.build_output_stream(
        *config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            if let Some(event) = cursor.fill(data, callback_control.volume()) {
                let _ = stream_tx.send(PlayerStreamEvent::Stream {
                    id: stream_id,
                    event,
                });
            }
        },
        |err| tracing::error!(?err, "A playback error has occured!"),
        None,
    )?;

    thread::spawn(move || {
        decode_loop(
            source,
            sample_rate,
            channels,
            chunk_tx,
            recycle_rx,
            position,
            control,
        )
    });
    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Harness<T> {
        cursor: ChunkCursor<T>,
        chunk_tx: SyncSender<Chunk<T>>,
        recycle_rx: Receiver<Vec<T>>,
    }

    fn harness<T>(channels: usize, sample_rate: u64) -> Harness<T> {
        let (chunk_tx, chunk_rx) = mpsc::sync_channel(QUEUED_CHUNKS);
        let (recycle_tx, recycle_rx) = mpsc::sync_channel(QUEUED_CHUNKS + 2);
        let cursor = ChunkCursor {
            chunk_rx,
            recycle_tx,
            position: Arc::new(StreamPosition::default()),
            channels,
            sample_rate,
            current: None,
            offset: 0,
            generation: 0,
            elapsed_secs: 0,
            ended: false,
        };
        Harness {
            cursor,
            chunk_tx,
            recycle_rx,
        }
    }

    fn chunk<T>(generation: u64, start_frame: u64, samples: Vec<T>) -> Chunk<T> {
        Chunk {
            generation,
            start_frame,
            samples,
        }
    }

    #[test]
    fn end_of_song_plays_silence_and_reports_once() {
        let Harness {
            mut cursor,
            chunk_tx,
            recycle_rx,
        } = harness(1, 10);
        chunk_tx.send(chunk(0, 0, vec![200u8; 3])).unwrap();
        drop(chunk_tx);

        let mut data = [0u8; 5];
        assert_eq!(cursor.fill(&mut data, 1.0), Some(StreamEvent::StreamEnded));
        assert_eq!(data, [200, 200, 200, 128, 128]);
        assert_eq!(cursor.fill(&mut data, 1.0), None);
        assert_eq!(data, [128; 5]);
        // the played buffer went back to the decode thread
        assert_eq!(recycle_rx.try_recv().unwrap().len(), 3);
    }

    #[test]
    fn underrun_plays_silence_without_ending() {
        let Harness {
            mut cursor,
            chunk_tx,
            ..
        } = harness(1, 10);
        let mut data = [1.0f32; 4];
        assert_eq!(cursor.fill(&mut data, 1.0), None);
        assert_eq!(data, [0.0; 4]);
        assert!(!cursor.ended);
        drop(chunk_tx);
    }

    #[test]
    fn chunks_from_before_a_seek_are_skipped() {
        let Harness {
            mut cursor,
            chunk_tx,
            ..
        } = harness(1, 10);
        chunk_tx.send(chunk(0, 0, vec![1i16; 4])).unwrap();
        let mut data = [0i16; 2];
        cursor.fill(&mut data, 1.0);
        assert_eq!(data, [1, 1]);

        // a seek happened: the rest of the old chunk and old queued chunks are dropped
        chunk_tx.send(chunk(0, 4, vec![2i16; 4])).unwrap();
        cursor.position.generation.store(1, Ordering::Release);
        chunk_tx.send(chunk(1, 50, vec![3i16; 4])).unwrap();
        cursor.fill(&mut data, 1.0);
        assert_eq!(data, [3, 3]);
        assert_eq!(cursor.position.played_frames.load(Ordering::Relaxed), 52);
    }

    #[test]
    fn newer_chunk_is_kept_even_if_seen_before_the_generation_bump() {
        let Harness {
            mut cursor,
            chunk_tx,
            ..
        } = harness(1, 10);
        chunk_tx.send(chunk(1, 50, vec![3i16; 4])).unwrap();
        let mut data = [0i16; 2];
        cursor.fill(&mut data, 1.0);
        assert_eq!(data, [3, 3]);
    }

    #[test]
    fn progress_is_reported_once_per_second() {
        let Harness {
            mut cursor,
            chunk_tx,
            ..
        } = harness(2, 10);
        chunk_tx.send(chunk(0, 0, vec![0.0f32; 60])).unwrap();
        let mut data = [0.0f32; 10];
        assert_eq!(cursor.fill(&mut data, 1.0), None);
        assert_eq!(
            cursor.fill(&mut data, 1.0),
            Some(StreamEvent::Progress(Duration::from_secs(1)))
        );
        assert_eq!(cursor.fill(&mut data, 1.0), None);
    }

    #[test]
    fn seek_targets() {
        // rewinding past the start
        assert_eq!(seek_target_ms(2000, -5000, None), 0);
        // fast-forward on a song shorter than a second
        assert_eq!(seek_target_ms(0, 5000, Some(500)), 0);
        // fast-forward stops a second short of the end
        assert_eq!(seek_target_ms(1000, 60_000, Some(10_000)), 9000);
        // ... but never moves backwards
        assert_eq!(seek_target_ms(9500, 5000, Some(10_000)), 9500);
        // unknown length
        assert_eq!(seek_target_ms(1000, 5000, None), 6000);
    }

    #[test]
    fn volume_scales_unsigned_around_midpoint() {
        assert_eq!(scale_volume(128u8, 0.5), 128);
        assert_eq!(scale_volume(255u8, 0.0), 128);
        assert_eq!(scale_volume(-1000i16, 0.5), -500);
    }
}
