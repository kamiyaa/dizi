use std::convert::From;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use shellexpand::tilde_with_context;

use super::{PlayerOption, PlayerOptionRaw};

fn default_socket_string() -> String {
    "~/dizi-server-socket".to_string()
}

fn default_playlist_string() -> String {
    "~/dizi-playlist.m3u".to_string()
}

fn default_socket_path() -> PathBuf {
    let s = default_socket_string();
    PathBuf::from(tilde_with_context(&s, dirs_next::home_dir).as_ref())
}

fn default_playlist_path() -> PathBuf {
    let s = default_playlist_string();
    PathBuf::from(tilde_with_context(&s, dirs_next::home_dir).as_ref())
}

fn default_audio_system() -> cpal::HostId {
    #[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))]
    {
        cpal::HostId::Alsa
    }
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        cpal::HostId::CoreAudio
    }
    #[cfg(target_os = "windows")]
    {
        cpal::HostId::Asio
    }
}

fn default_audio_system_string() -> String {
    "alsa".to_string()
}

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))]
fn str_to_cpal_hostid(s: &str) -> Option<cpal::HostId> {
    match s {
        "alsa" => Some(cpal::HostId::Alsa),
        #[cfg(feature = "jack")]
        "jack" => Some(cpal::HostId::Jack),
        _ => None,
    }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn str_to_cpal_hostid(s: &str) -> Option<cpal::HostId> {
    Some(cpal::HostId::CoreAudio)
}

#[cfg(target_os = "windows")]
fn str_to_cpal_hostid(s: &str) -> Option<cpal::HostId> {
    Some(cpal::HostId::Asio)
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfigRaw {
    #[serde(default = "default_socket_string")]
    pub socket: String,
    #[serde(default = "default_playlist_string")]
    pub playlist: String,
    #[serde(default = "default_audio_system_string")]
    pub audio_system: String,
    #[serde(default)]
    pub on_song_change: Option<String>,
    #[serde(default)]
    pub player: PlayerOptionRaw,
}

impl std::default::Default for ServerConfigRaw {
    fn default() -> Self {
        Self {
            socket: default_socket_string(),
            playlist: default_playlist_string(),
            audio_system: default_audio_system_string(),
            on_song_change: None,
            player: PlayerOptionRaw::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub socket: PathBuf,
    pub playlist: PathBuf,
    pub audio_system: cpal::HostId,
    pub on_song_change: Option<PathBuf>,
    pub player: PlayerOption,
}

impl ServerConfig {
    pub fn socket_ref(&self) -> &Path {
        self.socket.as_path()
    }
    pub fn playlist_ref(&self) -> &Path {
        self.playlist.as_path()
    }
    pub fn player_ref(&self) -> &PlayerOption {
        &self.player
    }
}

impl std::default::Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket: default_socket_path(),
            playlist: default_playlist_path(),
            audio_system: default_audio_system(),
            on_song_change: None,
            player: PlayerOption::default(),
        }
    }
}

impl From<ServerConfigRaw> for ServerConfig {
    fn from(raw: ServerConfigRaw) -> Self {
        let audio_system = str_to_cpal_hostid(&raw.audio_system.to_lowercase())
            .unwrap_or_else(default_audio_system);

        let socket = tilde_with_context(&raw.socket, dirs_next::home_dir);
        let playlist = tilde_with_context(&raw.playlist, dirs_next::home_dir);
        let on_song_change = raw
            .on_song_change
            .map(|path| PathBuf::from(tilde_with_context(&path, dirs_next::home_dir).as_ref()));

        Self {
            socket: PathBuf::from(socket.as_ref()),
            playlist: PathBuf::from(playlist.as_ref()),
            audio_system,
            on_song_change,
            player: PlayerOption::from(raw.player),
        }
    }
}
