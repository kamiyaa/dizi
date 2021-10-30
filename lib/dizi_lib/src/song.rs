use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time;

use rodio::decoder::Decoder;
use rodio::source::Source;

use serde_derive::{Deserialize, Serialize};

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
    pub fn new(path: &Path) -> DiziResult<Self> {
        let file = File::open(path)?;
        let buffer = BufReader::new(file);
        let source = Decoder::new(buffer)?;

        let audio_metadata = AudioMetadata::from_source(&source);
        let music_metadata = MusicMetadata {};

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
    _channels: u16,
    #[serde(rename = "sample_rate")]
    _sample_rate: u32,
    #[serde(rename = "total_duration")]
    _total_duration: Option<time::Duration>,
}

impl AudioMetadata {
    pub fn new(channels: u16, sample_rate: u32, total_duration: Option<time::Duration>) -> Self {
        Self {
            _channels: channels,
            _sample_rate: sample_rate,
            _total_duration: total_duration,
        }
    }

    pub fn channels(&self) -> u16 {
        self._channels
    }
    pub fn sample_rate(&self) -> u32 {
        self._sample_rate
    }
    pub fn total_duration(&self) -> Option<time::Duration> {
        self._total_duration
    }

    pub fn from_source<T: Source<Item = i16>>(source: &T) -> Self {
        let channels = source.channels();
        let sample_rate = source.sample_rate();
        let total_duration = source.total_duration();

        Self::new(channels, sample_rate, total_duration)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MusicMetadata {}
