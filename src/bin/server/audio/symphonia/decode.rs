use symphonia::core::audio::conv::ConvertibleSample;
use symphonia::core::codecs::audio::AudioDecoder;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatReader, SeekMode, SeekTo};
use symphonia::core::units::{Time, TimeBase};

use dizi::error::{AppError, AppResult};

/// The channel count and sample rate that symphonia actually produced when
/// decoding, as opposed to what was reported by the (possibly incomplete)
/// pre-decode probe metadata.
#[derive(Clone, Copy, Debug)]
pub struct DecodedAudioSpec {
    pub channels: usize,
    pub sample_rate: u32,
}

/// Decodes one track of a file, a packet at a time
pub struct TrackDecoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn AudioDecoder>,
    track_id: u32,
    time_base: Option<TimeBase>,
}

impl TrackDecoder {
    pub fn new(
        format: Box<dyn FormatReader>,
        decoder: Box<dyn AudioDecoder>,
        track_id: u32,
    ) -> Self {
        let time_base = format
            .tracks()
            .iter()
            .find(|t| t.id == track_id)
            .and_then(|t| t.time_base);
        Self {
            format,
            decoder,
            track_id,
            time_base,
        }
    }

    /// Decodes the next non-empty packet of the track into `buf`, replacing its contents.
    /// Returns `None` at the end of the track.
    ///
    /// The returned spec must be used (rather than pre-decode probe metadata) to configure
    /// playback, since some codecs/containers don't populate channel/sample-rate in their
    /// probed codec parameters until a packet is decoded; using stale or device-default
    /// values there causes the interleaved samples to be misread (e.g. a mono file misread
    /// as stereo plays back at 2x speed).
    pub fn decode_next<T>(&mut self, buf: &mut Vec<T>) -> AppResult<Option<DecodedAudioSpec>>
    where
        T: ConvertibleSample,
    {
        loop {
            let packet = match self.format.next_packet() {
                Ok(Some(packet)) => packet,
                Ok(None) => return Ok(None),
                Err(err) => {
                    tracing::debug!(?err, "Failed to read packet, ending track");
                    return Ok(None);
                }
            };
            // If the packet does not belong to the selected track, skip over it.
            if packet.track_id != self.track_id {
                continue;
            }

            match self.decoder.decode(&packet) {
                Ok(decoded) => {
                    if decoded.frames() == 0 {
                        continue;
                    }
                    let spec = DecodedAudioSpec {
                        channels: decoded.spec().channels().count(),
                        sample_rate: decoded.spec().rate(),
                    };
                    // resizes `buf`, so a reused buffer with enough capacity doesn't allocate
                    decoded.copy_to_vec_interleaved(buf);
                    return Ok(Some(spec));
                }
                Err(SymphoniaError::IoError(_)) | Err(SymphoniaError::DecodeError(_)) => continue,
                Err(err) => {
                    tracing::error!(?err, "Symphonia error");
                    return Err(AppError::from(err));
                }
            }
        }
    }

    /// Seeks to `time`. Returns the position actually reached, in frames at `sample_rate`.
    pub fn seek(&mut self, time: Time, sample_rate: u32) -> AppResult<u64> {
        let seeked = self.format.seek(
            SeekMode::Accurate,
            SeekTo::Time {
                time,
                track_id: Some(self.track_id),
            },
        )?;
        self.decoder.reset();

        let actual = self
            .time_base
            .and_then(|time_base| time_base.calc_time(seeked.actual_ts))
            .unwrap_or(time);
        let frames = actual.as_nanos().max(0) * sample_rate as i128 / 1_000_000_000;
        Ok(frames as u64)
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use symphonia::core::codecs::CodecParameters;
    use symphonia::core::units::Time;

    use dizi::song::DiziFile;

    use super::TrackDecoder;

    const SAMPLE_RATE: u32 = 8000;

    /// Writes a mono 16-bit PCM WAV file of `seconds` seconds
    fn write_wav(seconds: u32) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("dizi-test-{}.wav", uuid::Uuid::new_v4()));
        let data_len = SAMPLE_RATE * seconds * 2;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36 + data_len).to_le_bytes());
        bytes.extend_from_slice(b"WAVEfmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes()); // PCM
        bytes.extend_from_slice(&1u16.to_le_bytes()); // mono
        bytes.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
        bytes.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&16u16.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&data_len.to_le_bytes());
        bytes.resize(bytes.len() + data_len as usize, 0);
        std::fs::File::create(&path)
            .unwrap()
            .write_all(&bytes)
            .unwrap();
        path
    }

    fn open(path: &std::path::Path) -> TrackDecoder {
        let format = DiziFile::new(path).get_probe_result().unwrap();
        let track = format.tracks().first().unwrap();
        let track_id = track.id;
        let Some(CodecParameters::Audio(params)) = track.codec_params.as_ref() else {
            panic!("not audio");
        };
        let decoder = symphonia::default::get_codecs()
            .make_audio_decoder(params, &Default::default())
            .unwrap();
        TrackDecoder::new(format, decoder, track_id)
    }

    fn count_remaining_frames(decoder: &mut TrackDecoder) -> usize {
        let mut buf: Vec<f32> = Vec::new();
        let mut frames = 0;
        while decoder.decode_next(&mut buf).unwrap().is_some() {
            frames += buf.len();
        }
        frames
    }

    #[test]
    fn decodes_whole_track_in_packets() {
        let path = write_wav(2);
        let mut decoder = open(&path);
        let mut buf: Vec<f32> = Vec::new();
        let spec = decoder.decode_next(&mut buf).unwrap().unwrap();
        assert_eq!((spec.channels, spec.sample_rate), (1, SAMPLE_RATE));
        // a packet, not the whole file
        assert!(buf.len() < 2 * SAMPLE_RATE as usize);
        let total = buf.len() + count_remaining_frames(&mut decoder);
        assert_eq!(total, 2 * SAMPLE_RATE as usize);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn seek_reports_position_and_continues_from_there() {
        let path = write_wav(3);
        let mut decoder = open(&path);
        let frame = decoder
            .seek(Time::from_millis_u64(2000), SAMPLE_RATE)
            .unwrap();
        // accurate seeks land at or before the target
        assert!(frame <= 2 * SAMPLE_RATE as u64);
        let remaining = count_remaining_frames(&mut decoder) as u64;
        assert_eq!(frame + remaining, 3 * SAMPLE_RATE as u64);
        std::fs::remove_file(path).unwrap();
    }
}
