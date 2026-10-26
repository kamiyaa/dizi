mod impl_audio_player;

use std::path::PathBuf;
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle};

use cpal::traits::HostTrait;

use dizi::error::{AppError, AppErrorCode, AppResult};
use dizi::player::{PlayerState, PlayerStatus};
use dizi::playlist::PlaylistType;
use dizi::song::DiziAudioFile;

use crate::audio::device::get_default_host;
use crate::audio::request::PlayerRequest;
use crate::audio::symphonia::stream::{PlaybackControl, PlayerStream, PlayerStreamEvent};
use crate::config;
use crate::context::PlaylistContext;
use crate::events::AppEventSender;
use crate::playlist::DiziPlaylist;

#[derive(Debug)]
pub struct SymphoniaPlayer {
    pub state: PlayerState,
    pub playlist_context: PlaylistContext,

    pub player_req_tx: mpsc::Sender<PlayerStreamEvent>,
    pub player_res_rx: mpsc::Receiver<AppResult>,
    pub control: Arc<PlaybackControl>,

    pub _stream_handle: JoinHandle<AppResult>,
}

impl SymphoniaPlayer {
    pub fn new(config_t: &config::AppConfig, event_tx: AppEventSender) -> AppResult<Self> {
        let audio_host = get_default_host(config_t.server_ref().audio_system);
        let audio_device = audio_host.default_output_device().ok_or_else(|| {
            let error_msg = "Failed to get default output device";
            tracing::error!("{error_msg}");
            AppError::new(AppErrorCode::Symphonia, error_msg.to_string())
        })?;

        let server_config = config_t.server_ref();
        let player_config = server_config.player_ref();

        let control = Arc::new(PlaybackControl::new(player_config.volume as f32 / 100.0));

        // requests from this player and events from the output stream share one channel
        let (player_req_tx, stream_event_rx) = mpsc::channel();
        let (player_res_tx, player_res_rx) = mpsc::channel();

        let stream_handle: JoinHandle<AppResult> = {
            let stream_event_tx = player_req_tx.clone();
            let control = control.clone();
            thread::spawn(move || {
                let mut stream = PlayerStream::new(
                    event_tx,
                    stream_event_tx,
                    stream_event_rx,
                    player_res_tx,
                    control,
                    audio_device,
                )?;
                stream.listen_for_events()?;
                Ok(())
            })
        };

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
            volume: player_config.volume,
            audio_host: audio_host.id().name().to_lowercase(),
            ..PlayerState::default()
        };

        Ok(Self {
            state,
            playlist_context,
            player_req_tx,
            player_res_rx,
            control,
            _stream_handle: stream_handle,
        })
    }

    /// Sends `req` to the stream thread and waits for its result
    fn stream_request(&self, req: PlayerRequest) -> AppResult {
        self.player_req_tx.send(PlayerStreamEvent::Player(req))?;
        self.player_res_rx.recv()?
    }

    fn play(&mut self, song: &DiziAudioFile) -> AppResult {
        tracing::debug!(?song, "Playing song");

        self.stream_request(PlayerRequest::Play {
            song: Box::new(song.clone()),
        })?;

        self.state.status = PlayerStatus::Playing;
        self.state.song = Some(song.clone());
        Ok(())
    }

    fn set_playlist_type(&mut self, playlist_type: PlaylistType) {
        self.playlist_context.current_playlist_type = playlist_type;
        self.state.playlist_status = playlist_type;
    }
}
