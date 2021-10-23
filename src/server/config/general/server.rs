use std::convert::From;
use std::path;

use serde_derive::Deserialize;

use super::{PlayerOption, PlayerOptionCrude};

fn default_socket_path() -> path::PathBuf {
    path::PathBuf::from("/tmp/dizi-server-socket")
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
    #[serde(default = "default_socket_path")]
    pub socket: path::PathBuf,
    #[serde(default = "default_audio_system_string")]
    pub audio_system: String,
    #[serde(default)]
    pub player: PlayerOptionCrude,
}

impl std::default::Default for ServerConfigCrude {
    fn default() -> Self {
        Self {
            socket: default_socket_path(),
            audio_system: default_audio_system_string(),
            player: PlayerOptionCrude::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub socket: path::PathBuf,
    pub audio_system: cpal::HostId,
    pub player: PlayerOption,
}

impl ServerConfig {
    pub fn socket_ref(&self) -> &path::Path {
        self.socket.as_path()
    }
    pub fn player_ref(&self) -> &PlayerOption {
        &self.player
    }
}

impl std::default::Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket: default_socket_path(),
            audio_system: default_audio_system(),
            player: PlayerOption::default(),
        }
    }
}

impl From<ServerConfigCrude> for ServerConfig {
    fn from(crude: ServerConfigCrude) -> Self {
        let audio_system =
            str_to_cpal_hostid(&crude.audio_system).unwrap_or_else(default_audio_system);

        Self {
            socket: path::PathBuf::from(crude.socket),
            audio_system,
            player: PlayerOption::from(crude.player),
        }
    }
}
