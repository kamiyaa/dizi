use std::convert::From;
use std::path::{Path, PathBuf};

use serde_derive::Deserialize;
use shellexpand::tilde_with_context;

use super::{PlayerOption, PlayerOptionCrude};

fn default_socket_string() -> String {
    "/tmp/dizi-server-socket".to_string()
}

fn default_playlist_string() -> String {
    "/tmp/dizi-playlist.m3u".to_string()
}

fn default_socket_path() -> PathBuf {
    PathBuf::from(default_socket_string())
}

fn default_playlist_path() -> PathBuf {
    PathBuf::from(default_playlist_string())
}

fn default_audio_system() -> cpal::HostId {
    cpal::HostId::Jack
}

fn default_audio_system_string() -> String {
    "jack".to_string()
}

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd"))]
fn str_to_cpal_hostid(s: &str) -> Option<cpal::HostId> {
    match s {
        "jack" => Some(cpal::HostId::Jack),
        "alsa" => Some(cpal::HostId::Alsa),
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
pub struct ServerConfigCrude {
    #[serde(default = "default_socket_string")]
    pub socket: String,
    #[serde(default = "default_playlist_string")]
    pub playlist: String,
    #[serde(default = "default_audio_system_string")]
    pub audio_system: String,
    #[serde(default)]
    pub on_song_change: Option<String>,
    #[serde(default)]
    pub player: PlayerOptionCrude,
}

impl std::default::Default for ServerConfigCrude {
    fn default() -> Self {
        Self {
            socket: default_socket_string(),
            playlist: default_playlist_string(),
            audio_system: default_audio_system_string(),
            on_song_change: None,
            player: PlayerOptionCrude::default(),
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

impl From<ServerConfigCrude> for ServerConfig {
    fn from(crude: ServerConfigCrude) -> Self {
        let audio_system =
            str_to_cpal_hostid(&crude.audio_system).unwrap_or_else(default_audio_system);

        let socket = tilde_with_context(&crude.socket, dirs_next::home_dir);
        let playlist = tilde_with_context(&crude.playlist, dirs_next::home_dir);
        let on_song_change = match crude.on_song_change {
            Some(path) => Some(PathBuf::from(
                tilde_with_context(&path, dirs_next::home_dir).as_ref(),
            )),
            None => None,
        };

        Self {
            socket: PathBuf::from(socket.as_ref()),
            playlist: PathBuf::from(playlist.as_ref()),
            audio_system,
            on_song_change,
            player: PlayerOption::from(crude.player),
        }
    }
}
