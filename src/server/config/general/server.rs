use std::convert::From;
use std::path;

use serde_derive::Deserialize;
use shellexpand::tilde_with_context;

use super::{PlayerOption, PlayerOptionCrude};
use crate::HOME_DIR;

fn default_socket_string() -> String {
    "/tmp/dizi-server-socket".to_string()
}

fn default_playlist_string() -> String {
    "/tmp/dizi-playlist.m3u".to_string()
}

fn default_socket_path() -> path::PathBuf {
    path::PathBuf::from(default_socket_string())
}

fn default_playlist_path() -> path::PathBuf {
    path::PathBuf::from(default_playlist_string())
}

fn default_audio_system() -> cpal::HostId {
    cpal::HostId::Jack
}

fn default_audio_system_string() -> String {
    "jack".to_string()
}

fn str_to_cpal_hostid(s: &str) -> Option<cpal::HostId> {
    match s {
        "jack" => Some(cpal::HostId::Jack),
        "alsa" => Some(cpal::HostId::Alsa),
        _ => None,
    }
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
    pub player: PlayerOptionCrude,
}

impl std::default::Default for ServerConfigCrude {
    fn default() -> Self {
        Self {
            socket: default_socket_string(),
            playlist: default_playlist_string(),
            audio_system: default_audio_system_string(),
            player: PlayerOptionCrude::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub socket: path::PathBuf,
    pub playlist: path::PathBuf,
    pub audio_system: cpal::HostId,
    pub player: PlayerOption,
}

impl ServerConfig {
    pub fn socket_ref(&self) -> &path::Path {
        self.socket.as_path()
    }
    pub fn playlist_ref(&self) -> &path::Path {
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

        Self {
            socket: path::PathBuf::from(socket.as_ref()),
            playlist: path::PathBuf::from(playlist.as_ref()),
            audio_system,
            player: PlayerOption::from(crude.player),
        }
    }
}
