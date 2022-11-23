use std::collections::HashMap;
#[cfg(not(feature = "symphonia-backend"))]
use std::fs::File;
#[cfg(not(feature = "symphonia-backend"))]
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::time;

#[cfg(feature = "rodio-backend")]
use rodio::decoder::Decoder;
#[cfg(feature = "rodio-backend")]
use rodio::source::Source;

#[cfg(feature = "symphonia-backend")]
use symphonia::core::codecs::{CodecParameters, CODEC_TYPE_NULL};
#[cfg(feature = "symphonia-backend")]
use symphonia::core::formats::FormatOptions;
#[cfg(feature = "symphonia-backend")]
use symphonia::core::io::MediaSourceStream;
#[cfg(feature = "symphonia-backend")]
use symphonia::core::meta::{MetadataOptions, MetadataRevision};
#[cfg(feature = "symphonia-backend")]
use symphonia::core::probe::Hint;

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
    #[serde(rename = "metadata_loaded")]
    _metadata_loaded: bool,
}

impl Song {
    #[cfg(feature = "rodio-backend")]
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
            _metadata_loaded: false,
            _audio_metadata: audio_metadata,
            _music_metadata: music_metadata,
        })
    }
    #[cfg(feature = "symphonia-backend")]
    pub fn new(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap()
            .into_owned();
        let mut song = Self {
            _file_name: file_name,
            _path: path.to_path_buf(),
            _metadata_loaded: false,
            _audio_metadata: AudioMetadata::default(),
            _music_metadata: MusicMetadata::default(),
        };
        let _ = song.load_metadata();
        song
    }

    pub fn load_metadata(&mut self) -> DiziResult {
        let mut hint = Hint::new();
        if let Some(ext) = self.file_path().extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        };

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let src = std::fs::File::open(self.file_path())?;

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)?;

        // Get the instantiated format reader.
        let mut format = probed.format;

        let audio_metadata = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .map(|track| AudioMetadata::from(&track.codec_params))
            .unwrap_or_else(|| AudioMetadata::default());

        let music_metadata = format
            .metadata()
            .skip_to_latest()
            .map(|metadata| MusicMetadata::from(metadata))
            .unwrap_or_else(|| MusicMetadata::default());

        self._audio_metadata = audio_metadata;
        self._music_metadata = music_metadata;
        self._metadata_loaded = true;
        Ok(())
    }

    pub fn metadata_loaded(&self) -> bool {
        self._metadata_loaded
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
    pub channels: Option<usize>,
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

#[cfg(feature = "rodio-backend")]
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
        let channels = source.channels.map(|c| c.count());
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
    pub standard_tags: HashMap<String, String>,
    pub tags: HashMap<String, String>,
}

#[cfg(feature = "symphonia-backend")]
impl std::convert::From<&MetadataRevision> for MusicMetadata {
    fn from(metadata: &MetadataRevision) -> Self {
        let standard_tags: HashMap<String, String> = metadata
            .tags()
            .iter()
            .filter_map(|tag| {
                tag.std_key
                    .map(|std_key| (format!("{:?}", std_key), tag.value.to_string()))
            })
            .collect();
        let tags: HashMap<String, String> = metadata
            .tags()
            .iter()
            .filter(|tag| tag.std_key.is_none())
            .map(|tag| (tag.key.to_owned(), tag.value.to_string()))
            .collect();
        Self {
            standard_tags,
            tags,
        }
    }
}
