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

/// Setup a unix socket
pub fn setup_socket(config: &AppConfig) -> DiziResult<UnixListener> {
    let socket = Path::new(config.server_ref().socket_ref());
    if socket.exists() {
        fs::remove_file(socket)?;
    }
    let stream = UnixListener::bind(socket)?;
    Ok(stream)
}

/// run server
pub fn run(config: AppConfig) -> DiziResult {
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

        tracing::debug!(?event, "Received server event");
        match event {
            AppEvent::Client { uuid, request } => {
                let res = server_util::process_client_request(&mut context, &uuid, &request);
                if let Err(err) = res {
                    tracing::debug!(?err, ?uuid, ?request, "Failed to process client request");
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
                    tracing::debug!(?err, "Failed to process server event");
                }
            }
        }
    }

    let playlist_path = context.config_ref().server_ref().playlist_ref();
    let playlist = &context.player.playlist_context.file_playlist;

    tracing::debug!(?playlist_path, "Saving playlist");

    let mut file = std::fs::File::create(playlist_path)?;
    let mut writer = m3u::Writer::new(&mut file);
    for song in playlist.contents.iter() {
        let entry = m3u::Entry::Path(song.file_path().to_path_buf());
        writer.write_entry(&entry)?;
    }
    tracing::debug!(?playlist_path, "Playlist saved!");

    // broadcast to all clients that the server has exited
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
