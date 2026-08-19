use std::iter::Iterator;

use symphonia::core::audio::conv::ConvertibleSample;
use symphonia::core::codecs::audio::AudioDecoder;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatReader;
use symphonia::core::packet::Packet;

use dizi::error::{AppResult, DiziError};

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
            let next_packet = self.format.next_packet().ok()??;

            // If the packet does not belong to the selected track, skip over it.
            if next_packet.track_id != self.track_id {
                continue;
            }
            return Some(next_packet);
        }
    }
}

/// The channel count and sample rate that symphonia actually produced when
/// decoding, as opposed to what was reported by the (possibly incomplete)
/// pre-decode probe metadata.
#[derive(Clone, Copy, Debug)]
pub struct DecodedAudioSpec {
    pub channels: usize,
    pub sample_rate: u32,
}

pub struct PacketDecoder {
    decoder: Box<dyn AudioDecoder>,
}

impl PacketDecoder {
    pub fn new(decoder: Box<dyn AudioDecoder>) -> Self {
        Self { decoder }
    }

    pub fn decode<T>(&mut self, packet: Packet) -> AppResult<(Vec<T>, Option<DecodedAudioSpec>)>
    where
        T: ConvertibleSample + cpal::Sample + Send + 'static,
    {
        // Decode the packet into audio samples.
        match self.decoder.decode(&packet) {
            Ok(decoded) => {
                if decoded.frames() > 0 {
                    let spec = DecodedAudioSpec {
                        channels: decoded.spec().channels().count(),
                        sample_rate: decoded.spec().rate(),
                    };
                    let mut sample_data = Vec::with_capacity(decoded.frames());
                    decoded.copy_to_vec_interleaved(&mut sample_data);
                    Ok((sample_data, Some(spec)))
                } else {
                    Ok((vec![], None))
                }
            }
            Err(SymphoniaError::IoError(_)) => Ok((vec![], None)),
            Err(SymphoniaError::DecodeError(_)) => Ok((vec![], None)),
            Err(err) => {
                tracing::error!(?err, "Symphonia error");
                Err(DiziError::from(err))
            }
        }
    }
}

/// Decodes every packet from `reader`, concatenating the resulting samples.
///
/// Also returns the channel count and sample rate symphonia actually decoded
/// the audio at, taken from the first non-empty decoded packet. This must be
/// used (rather than pre-decode probe metadata) to configure playback,
/// since some codecs/containers don't populate channel/sample-rate in their
/// probed codec parameters until a packet is decoded; using stale or
/// device-default values there causes the interleaved sample buffer to be
/// misread (e.g. a mono file misread as stereo plays back at 2x speed).
pub fn decode_all<T>(
    reader: PacketReader,
    mut decoder: PacketDecoder,
) -> AppResult<(Vec<T>, Option<DecodedAudioSpec>)>
where
    T: ConvertibleSample + cpal::Sample + Send + 'static,
{
    let mut samples = Vec::new();
    let mut spec = None;
    for packet in reader {
        let (packet_samples, packet_spec) = decoder.decode::<T>(packet)?;
        if spec.is_none() {
            spec = packet_spec;
        }
        samples.extend(packet_samples);
    }
    Ok((samples, spec))
}
