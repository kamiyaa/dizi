use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time;

use symphonia::core::codecs::{CodecParameters, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, MetadataRevision};
use symphonia::core::probe::Hint;

use serde::{Deserialize, Serialize};

use crate::error::{DiziError, DiziResult};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DiziSongEntry {
    Unloaded(DiziFile),
    Loaded(DiziAudioFile),
}

impl DiziSongEntry {
    pub fn load_metadata(self) -> DiziResult<DiziAudioFile> {
        match self {
            Self::Unloaded(s) => DiziAudioFile::try_from(s),
            Self::Loaded(s) => Ok(s),
        }
    }
    pub fn file_path(&self) -> &Path {
        match self {
            Self::Unloaded(s) => &s.file_path,
            Self::Loaded(s) => &s.file.file_path,
        }
    }

    pub fn file_name(&self) -> &str {
        match self {
            Self::Unloaded(s) => &s.file_name,
            Self::Loaded(s) => &s.file.file_name,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiziFile {
    pub file_name: String,
    pub file_path: PathBuf,
}

impl DiziFile {
    pub fn new(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_default()
            .into_owned();
        Self {
            file_name,
            file_path: path.to_path_buf(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiziAudioFile {
    pub file: DiziFile,
    pub audio_metadata: AudioMetadata,
    pub music_metadata: MusicMetadata,
}

impl TryFrom<DiziFile> for DiziAudioFile {
    type Error = DiziError;
    fn try_from(value: DiziFile) -> Result<Self, Self::Error> {
        tracing::debug!("Loading metadata for {:?}", value.file_path);
        let mut hint = Hint::new();
        if let Some(ext) = value.file_path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        };

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let src = std::fs::File::open(&value.file_path)?;

        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        // get probe
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
        Ok(Self {
            file: value,
            audio_metadata,
            music_metadata,
        })
    }
}

impl DiziAudioFile {
    pub fn file_path(&self) -> &Path {
        self.file.file_path.as_path()
    }

    pub fn file_name(&self) -> &str {
        &self.file.file_name
    }

    pub fn audio_metadata(&self) -> &AudioMetadata {
        &self.audio_metadata
    }

    pub fn music_metadata(&self) -> &MusicMetadata {
        &self.music_metadata
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
