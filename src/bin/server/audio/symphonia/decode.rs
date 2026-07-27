use std::iter::Iterator;

use symphonia::core::codecs::audio::AudioDecoder;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatReader;
use symphonia::core::packet::Packet;

use dizi::error::{DiziError, DiziResult};

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

pub struct PacketDecoder {
    decoder: Box<dyn AudioDecoder>,
}

impl PacketDecoder {
    pub fn new(decoder: Box<dyn AudioDecoder>) -> Self {
        Self { decoder }
    }

    pub fn decode<T>(&mut self, packet: Packet) -> DiziResult<Vec<T>>
    where
        T: symphonia::core::audio::sample::Sample
            + cpal::Sample
            + std::marker::Send
            + 'static
            + symphonia::core::audio::conv::FromSample<i8>
            + symphonia::core::audio::conv::FromSample<i16>
            + symphonia::core::audio::conv::FromSample<i32>
            + symphonia::core::audio::conv::FromSample<u8>
            + symphonia::core::audio::conv::FromSample<u16>
            + symphonia::core::audio::conv::FromSample<u32>
            + symphonia::core::audio::conv::FromSample<f32>
            + symphonia::core::audio::conv::FromSample<f64>
            + symphonia::core::audio::conv::FromSample<symphonia::core::audio::sample::i24>
            + symphonia::core::audio::conv::FromSample<symphonia::core::audio::sample::u24>,
    {
        // Decode the packet into audio samples.
        match self.decoder.decode(&packet) {
            Ok(decoded) => {
                if decoded.frames() > 0 {
                    let mut sample_data = Vec::with_capacity(decoded.frames());
                    decoded.copy_to_vec_interleaved(&mut sample_data);
                    Ok(sample_data)
                } else {
                    Ok(vec![])
                }
            }
            Err(SymphoniaError::IoError(_)) => Ok(vec![]),
            Err(SymphoniaError::DecodeError(_)) => Ok(vec![]),
            Err(err) => {
                tracing::error!(?err, "Symphonia error");
                Err(DiziError::from(err))
            }
        }
    }
}
