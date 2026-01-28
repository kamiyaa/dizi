use std::iter::Iterator;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::Decoder;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::{FormatReader, Packet};

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
            let packet = match self.format.next_packet() {
                Ok(p) => p,
                Err(_) => return None,
            };

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
                tracing::error!(?err, "Symphonia error");
                Err(DiziError::from(err))
            }
        }
    }
}
