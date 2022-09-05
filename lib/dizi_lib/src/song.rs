use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time;

#[cfg(not(feature = "symphonia-backend"))]
use rodio::decoder::Decoder;
#[cfg(not(feature = "symphonia-backend"))]
use rodio::source::Source;

#[cfg(feature = "symphonia-backend")]
use symphonia::core::codecs::{CodecParameters, DecoderOptions, CODEC_TYPE_NULL};
#[cfg(feature = "symphonia-backend")]
use symphonia::core::errors::Error;
#[cfg(feature = "symphonia-backend")]
use symphonia::core::formats::FormatOptions;
#[cfg(feature = "symphonia-backend")]
use symphonia::core::io::MediaSourceStream;
#[cfg(feature = "symphonia-backend")]
use symphonia::core::meta::MetadataOptions;
use symphonia::core::meta::MetadataRevision;
#[cfg(feature = "symphonia-backend")]
use symphonia::core::probe::Hint;

use serde_derive::{Deserialize, Serialize};
use symphonia::core::units::TimeBase;

use crate::error::DiziResult;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    _file_name: String,
    #[serde(rename = "path")]
    _path: PathBuf,
    #[serde(rename = "audio_metadata")]
    _audio_metadata: AudioMetadata,
    #[serde(rename = "music_metadata")]
    _music_metadata: MusicMetadata,
}

impl Song {
    #[cfg(not(feature = "symphonia-backend"))]
    pub fn new(path: &Path) -> DiziResult<Self> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let source = Decoder::new(buffer)?;

        let audio_metadata = AudioMetadata::from_source(&source);
        let music_metadata = MusicMetadata::default();

        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap()
            .into_owned();

        Ok(Self {
            _file_name: file_name,
            _path: path.to_path_buf(),
            _audio_metadata: audio_metadata,
            _music_metadata: music_metadata,
        })
    }
    #[cfg(feature = "symphonia-backend")]
    pub fn new(path: &Path) -> DiziResult<Self> {
        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        };

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let src = std::fs::File::open(&path).expect("failed to open media");

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("unsupported format");

        // Get the instantiated format reader.
        let mut format = probed.format;

        let audio_metadata = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .map(|track| AudioMetadata::from(&track.codec_params))
            .unwrap_or_else(|| AudioMetadata::default());

        let music_metadata = format.metadata().skip_to_latest()
            .map(|metadata| MusicMetadata::from(metadata))
            .unwrap_or_else(|| MusicMetadata::default());

        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap()
            .into_owned();
        Ok(Self {
            _file_name: file_name,
            _path: path.to_path_buf(),
            _audio_metadata: audio_metadata,
            _music_metadata: music_metadata,
        })
    }


    pub fn file_path(&self) -> &Path {
        self._path.as_path()
    }

    pub fn file_name(&self) -> &str {
        &self._file_name
    }

    pub fn audio_metadata(&self) -> &AudioMetadata {
        &self._audio_metadata
    }

    pub fn music_metadata(&self) -> &MusicMetadata {
        &self._music_metadata
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioMetadata {
    #[serde(rename = "channels")]
    pub channels: Option<u16>,
    #[serde(rename = "sample_rate")]
    pub sample_rate: Option<u32>,
    #[serde(rename = "total_duration")]
    pub total_duration: Option<time::Duration>,
}

impl std::default::Default for AudioMetadata {
    fn default() -> Self {
        Self {
            channels: None,
            sample_rate: None,
            total_duration: None,
        }
    }
}

#[cfg(not(feature = "symphonia-backend"))]
impl std::convert::From<Source<Item = i16>> for AudioMetadata {
    fn from(source: Source<Item = i16>) -> Self {
        let channels = Some(source.channels());
        let sample_rate = Some(source.sample_rate());
        let total_duration = source.total_duration();

        Self {
            channels,
            sample_rate,
            total_duration,
        }
    }
}

#[cfg(feature = "symphonia-backend")]
impl std::convert::From<&CodecParameters> for AudioMetadata {
    fn from(source: &CodecParameters) -> Self {
        let channels = Some(2);
        let sample_rate = source.sample_rate;

        let total_duration = match (source.time_base, source.n_frames) {
            (Some(time_base), Some(n_frames)) => {
                let unit_time = time_base.calc_time(n_frames);
                let duration = time::Duration::from_secs(unit_time.seconds);
                Some(duration)
            }
            _ => None,
        };

        Self {
            channels,
            sample_rate,
            total_duration,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MusicMetadata {
    pub tags: HashMap<String, String>,
}

#[cfg(feature = "symphonia-backend")]
impl std::convert::From<&MetadataRevision> for MusicMetadata {
    fn from(metadata: &MetadataRevision) -> Self {
        let tags: HashMap<String, String> = metadata.tags()
            .iter()
            .map(|tag| (tag.key.to_owned(), tag.value.to_string()))
            .collect();
        Self {
            tags
        }
    }
}
