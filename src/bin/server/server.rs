use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::thread;

use dizi::error::DiziResult;
use dizi::response::server::ServerBroadcastEvent;

use crate::audio::symphonia::player::SymphoniaPlayer;
use crate::config::AppConfig;
use crate::context::{AppContext, QuitType};
use crate::events::{AppEvent, Events, ServerEvent, ServerEventSender};
use crate::server_util;

pub fn setup_socket(config: &AppConfig) -> DiziResult<UnixListener> {
    let socket = Path::new(config.server_ref().socket_ref());
    if socket.exists() {
        fs::remove_file(socket)?;
    }
    let stream = UnixListener::bind(socket)?;
    Ok(stream)
}

pub fn serve(config: AppConfig) -> DiziResult {
    let events = Events::new();

    let player = {
        let server_event_tx = events.server_event_sender().clone();
        SymphoniaPlayer::new(&config, server_event_tx)?
    };

    let mut context = AppContext {
        events,
        config,
        quit: QuitType::DoNot,
        player,
    };

    let listener = setup_socket(context.config_ref())?;
    // thread for listening to new client connections
    {
        let server_event_tx = context.events.server_event_sender().clone();
        thread::spawn(|| listen_for_clients(listener, server_event_tx));
    }

    while context.quit == QuitType::DoNot {
        let event = match context.events.next() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        tracing::debug!("Server Event: {:?}", event);

        match event {
            AppEvent::Client { uuid, request } => {
                let res = server_util::process_client_request(&mut context, &uuid, request);
                if let Err(err) = res {
                    tracing::debug!("Error: {:?}", err);
                    context
                        .events
                        .broadcast_event(ServerBroadcastEvent::ServerError {
                            msg: err.to_string(),
                        });
                }
            }
            AppEvent::Server(event) => {
                let res = server_util::process_server_event(&mut context, event);
                if let Err(err) = res {
                    tracing::debug!("Error: {:?}", err);
                }
            }
        }
    }

    let playlist_path = context.config_ref().server_ref().playlist_ref();
    let playlist = &context.player.playlist_context.file_playlist;

    tracing::debug!("Saving playlist to '{}'", playlist_path.to_string_lossy());

    let mut file = std::fs::File::create(playlist_path)?;
    let mut writer = m3u::Writer::new(&mut file);
    for song in playlist.contents.iter() {
        let entry = m3u::Entry::Path(song.file_path().to_path_buf());
        writer.write_entry(&entry)?;
    }
    tracing::debug!("Playlist saved!");

    context
        .events
        .broadcast_event(ServerBroadcastEvent::ServerQuit);

    Ok(())
}

pub fn listen_for_clients(listener: UnixListener, event_tx: ServerEventSender) -> DiziResult {
    for stream in listener.incoming().flatten() {
        let _ = event_tx.send(ServerEvent::NewClient(stream));
    }
    Ok(())
}
