use std::path;

use serde_derive::Deserialize;

use crate::config::{parse_to_config_file, ConfigStructure, Flattenable};

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
pub struct RawServerConfig {
    #[serde(default = "default_socket_path")]
    pub socket: path::PathBuf,
    #[serde(default = "default_audio_system_string")]
    pub audio_system: String,
}

impl std::default::Default for RawServerConfig {
    fn default() -> Self {
        Self {
            socket: default_socket_path(),
            audio_system: default_audio_system_string(),
        }
    }
}

impl Flattenable<ServerConfig> for RawServerConfig {
    fn flatten(self) -> ServerConfig {
        let audio_system =
            str_to_cpal_hostid(&self.audio_system).unwrap_or_else(default_audio_system);

        ServerConfig {
            socket: path::PathBuf::from(self.socket),
            audio_system,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub socket: path::PathBuf,
    pub audio_system: cpal::HostId,
}

impl std::default::Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket: default_socket_path(),
            audio_system: default_audio_system(),
        }
    }
}

impl ConfigStructure for ServerConfig {
    fn get_config(file_name: &str) -> Self {
        parse_to_config_file::<RawServerConfig, ServerConfig>(file_name)
            .unwrap_or_else(Self::default)
    }
}
