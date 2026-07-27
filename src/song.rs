use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time;

use symphonia::core::codecs::CodecParameters;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, MetadataRevision, RawValue};

use serde::{Deserialize, Serialize};

use crate::error::{DiziError, DiziErrorKind, DiziResult};

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
    pub file_ext: Option<String>,
}

impl DiziFile {
    pub fn new(path: &Path) -> Self {
        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_default()
            .into_owned();
        let file_ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_string());

        Self {
            file_name,
            file_path: path.to_path_buf(),
            file_ext,
        }
    }

    pub fn get_probe_result(&self) -> DiziResult<Box<dyn FormatReader>> {
        let mut hint = Hint::new();
        if let Some(ext) = self.file_ext.as_ref() {
            hint.with_extension(ext);
        };

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let src = std::fs::File::open(&self.file_path)?;
        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        // get probe
        let probed = symphonia::default::get_probe().probe(&hint, mss, fmt_opts, meta_opts)?;
        Ok(probed)
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
        tracing::debug!(file_path=?value.file_path, "Loading metadata");
        let mut hint = Hint::new();
        if let Some(ext) = value.file_ext.as_ref() {
            hint.with_extension(ext);
        };

        // Use the default options for metadata and format readers.
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let src = std::fs::File::open(&value.file_path)?;
        // Create the media source stream.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());
        // get probe
        let mut probed = symphonia::default::get_probe().probe(&hint, mss, fmt_opts, meta_opts)?;

        let audio_metadata = AudioMetadata::from_format_reader(probed.as_ref())?;

        let music_metadata = probed
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
    #[serde(rename = "track_id")]
    pub track_id: u32,
    #[serde(rename = "bit_depth")]
    pub bit_depth: u32,
    #[serde(rename = "channels")]
    pub channels: Option<usize>,
    #[serde(rename = "sample_rate")]
    pub sample_rate: Option<u32>,
    #[serde(rename = "total_duration")]
    pub total_duration: Option<time::Duration>,
}

impl AudioMetadata {
    pub fn from_format_reader(reader: &dyn FormatReader) -> DiziResult<Self> {
        let media_info = reader.media_info();

        let start_timestamp = media_info.start_ts;

        let total_duration = match (media_info.time_base, media_info.duration) {
            (Some(time_base), Some(duration)) => {
                let end_timestamp = start_timestamp.saturating_add(duration);
                let unit_time = time_base.calc_time(end_timestamp).ok_or_else(|| {
                    let error_msg = "Failed to calculate time";
                    tracing::error!(?time_base, ?start_timestamp, ?duration, "{error_msg}");
                    DiziError::new(DiziErrorKind::ParseError, error_msg.to_string())
                })?;
                let duration = time::Duration::from_secs(unit_time.as_secs() as u64);
                Some(duration)
            }
            _ => None,
        };

        let track = reader.tracks().get(0).ok_or_else(|| {
            let error_msg = "No tracks found";
            tracing::error!("{error_msg}");
            DiziError::new(DiziErrorKind::ParseError, error_msg.to_string())
        })?;

        let track_id = track.id;
        let codec_parameters = track.codec_params.as_ref().ok_or_else(|| {
            let error_msg = "No codec parameters found";
            tracing::error!("{error_msg}");
            DiziError::new(DiziErrorKind::ParseError, error_msg.to_string())
        })?;

        let audio_codec_params = match codec_parameters {
            CodecParameters::Audio(params) => params,
            _ => {
                let error_msg = "Codec not audio";
                tracing::error!("{error_msg}");
                let err = DiziError::new(DiziErrorKind::ParseError, error_msg.to_string());
                return Err(err);
            }
        };

        let channels = audio_codec_params.channels.as_ref().map(|c| c.count());
        let sample_rate = audio_codec_params.sample_rate;

        let bit_depth = audio_codec_params.bits_per_sample.unwrap_or(16);
        Ok(Self {
            track_id,
            bit_depth,
            channels,
            sample_rate,
            total_duration,
        })
    }
}

impl std::default::Default for AudioMetadata {
    fn default() -> Self {
        Self {
            track_id: 0,
            bit_depth: 16,
            channels: None,
            sample_rate: None,
            total_duration: None,
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
            .media
            .tags
            .iter()
            .filter_map(|tag| {
                let std_key = tag.std.clone()?;
                let tag_value = match &tag.raw.value {
                    RawValue::String(s) => Some(s.to_string()),
                    _ => None,
                }?;
                Some((format!("{:?}", std_key), tag_value))
            })
            .collect();
        let tags: HashMap<String, String> = metadata
            .media
            .tags
            .iter()
            .filter(|tag| !tag.has_std_tag())
            .map(|tag| {
                let tag_key = tag.raw.key.clone();
                let tag_value = match &tag.raw.value {
                    RawValue::String(s) => s.to_string(),
                    _ => String::new(),
                };
                (tag_key, tag_value)
            })
            .collect();
        Self {
            standard_tags,
            tags,
        }
    }
}
