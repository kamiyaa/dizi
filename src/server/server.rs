use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;
use dizi_lib::response::server::ServerBroadcastEvent;

use crate::client;
use crate::config::default::AppConfig;
use crate::context::{AppContext, QuitType};
use crate::events::{AppEvent, ServerEvent, ServerEventSender};
use crate::server_command::run_command;
use crate::server_commands::{player_play, player_play_next};

pub fn setup_socket(config: &AppConfig) -> DiziResult<UnixListener> {
    let socket = Path::new(config.server_ref().socket.as_path());
    if socket.exists() {
        fs::remove_file(&socket)?;
    }
    let stream = UnixListener::bind(&socket)?;
    Ok(stream)
}

pub fn serve(config: AppConfig) -> DiziResult<()> {
    let mut context = AppContext::new(config);

    let listener = setup_socket(context.config_ref())?;
    {
        // thread for listening to new client connections
        let server_tx2 = context.events.server_event_sender().clone();
        thread::spawn(|| listen_for_clients(listener, server_tx2));
    }

    while context.quit == QuitType::DoNot {
        let event = match context.events.next() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        match event {
            AppEvent::Client(event) => {
                let _ = run_command(&mut context, event);
            }
            AppEvent::Server(event) => {
                process_server_event(&mut context, event);
            }
        }
    }

    Ok(())
}

pub fn process_server_event(context: &mut AppContext, event: ServerEvent) {
    match event {
        ServerEvent::NewClient(stream) => {
            let client_tx2 = context.events.client_request_sender().clone();
            let (server_tx, server_rx) = mpsc::channel();

            thread::spawn(|| client::handle_client(stream, client_tx2, server_rx));
            context.events.add_broadcast_listener(server_tx);
        }
        ServerEvent::PlayerProgressUpdate(elapsed) => {
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerProgressUpdate { elapsed });
        }
        ServerEvent::PlayerDone => {
            process_done_song(context);
        }
    }
}

pub fn listen_for_clients(listener: UnixListener, event_tx: ServerEventSender) -> DiziResult<()> {
    for stream in listener.incoming().flatten() {
        event_tx.send(ServerEvent::NewClient(stream));
    }
    Ok(())
}

pub fn process_done_song(context: &mut AppContext) {
    let next_enabled = context.player_context_ref().player_ref().next_enabled();
    let repeat_enabled = context.player_context_ref().player_ref().repeat_enabled();

    if !next_enabled {
        if repeat_enabled {
            eprintln!("Replaying song!");
            let song = context
                .player_context_mut()
                .player_mut()
                .current_song_ref()
                .map(|s| s.clone());
            if let Some(song) = song {
                player_play(context, song.file_path());
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { song });
            }
        } else {
            eprintln!("Done playing song!");
        }
    } else {
        player_play_next(context);
        if let Some(song) = context.player_context_ref().player_ref().current_song_ref() {
            let song = song.clone();
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { song });
        }
    }
}
