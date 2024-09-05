mod impl_audio_player;

use std::path::PathBuf;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

use cpal::traits::HostTrait;

use dizi::error::{DiziError, DiziErrorKind, DiziResult};
use dizi::player::{PlayerState, PlayerStatus};
use dizi::playlist::PlaylistType;
use dizi::song::DiziAudioFile;

use crate::audio::device::get_default_host;
use crate::audio::request::PlayerRequest;
use crate::audio::symphonia::stream::PlayerStream;
use crate::config;
use crate::context::PlaylistContext;
use crate::events::ServerEventSender;
use crate::playlist::DiziPlaylist;
use crate::traits::AudioPlayer;

#[derive(Debug)]
pub struct SymphoniaPlayer {
    pub state: PlayerState,
    pub playlist_context: PlaylistContext,

    pub player_req_tx: mpsc::Sender<PlayerRequest>,
    pub player_res_rx: mpsc::Receiver<DiziResult>,

    pub _stream_handle: JoinHandle<DiziResult>,
}

impl SymphoniaPlayer {
    pub fn new(config_t: &config::AppConfig, event_tx: ServerEventSender) -> DiziResult<Self> {
        let audio_host = get_default_host(config_t.server_ref().audio_system);
        let audio_device = audio_host.default_output_device().ok_or_else(|| {
            let error_msg = "Failed to get default output device";
            tracing::error!("{error_msg}");
            DiziError::new(DiziErrorKind::Symphonia, error_msg.to_string())
        })?;

        let (player_req_tx, player_req_rx) = mpsc::channel();
        let (player_res_tx, player_res_rx) = mpsc::channel();

        let stream_handle: JoinHandle<DiziResult> = thread::spawn(move || {
            let mut stream =
                PlayerStream::new(event_tx, player_res_tx, player_req_rx, audio_device)?;
            stream.listen_for_events()?;
            Ok(())
        });

        let server_config = config_t.server_ref();
        let player_config = server_config.player_ref();

        let playlist_context = PlaylistContext {
            file_playlist: DiziPlaylist::from_file(
                &PathBuf::from("/"),
                server_config.playlist_ref(),
            )
            .unwrap_or_default(),
            ..Default::default()
        };
        let state = PlayerState {
            next: player_config.next,
            repeat: player_config.repeat,
            shuffle: player_config.shuffle,
            volume: config_t.server_ref().player_ref().volume,
            audio_host: audio_host.id().name().to_lowercase(),
            ..PlayerState::default()
        };

        Ok(Self {
            state,
            playlist_context,
            player_req_tx,
            player_res_rx,
            _stream_handle: stream_handle,
        })
    }

    fn player_stream_req(&self) -> &mpsc::Sender<PlayerRequest> {
        &self.player_req_tx
    }
    fn player_stream_res(&self) -> &mpsc::Receiver<DiziResult> {
        &self.player_res_rx
    }

    fn play(&mut self, song: &DiziAudioFile) -> DiziResult {
        tracing::debug!("Song: {:#?}", song);

        self.player_stream_req().send(PlayerRequest::Play {
            song: song.clone(),
            volume: self.get_volume() as f32 / 100.0,
        })?;

        self.player_stream_res().recv()??;

        self.state.status = PlayerStatus::Playing;
        self.state.song = Some(song.clone());
        Ok(())
    }

    fn set_playlist_type(&mut self, playlist_type: PlaylistType) {
        self.playlist_context.current_playlist_type = playlist_type;
        self.state.playlist_status = playlist_type;
    }
}
