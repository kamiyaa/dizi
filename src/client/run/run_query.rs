use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::thread;

use strfmt::strfmt;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_lib::request::client::ClientRequest;
use dizi_lib::response::server::ServerBroadcastEvent;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::util::request::send_client_request;

pub fn run_query(context: &mut AppContext, query: String) -> DiziResult<()> {
    // server listener
    {
        let stream = context.clone_stream()?;
        let event_tx = context.events.event_tx.clone();

        let _ = thread::spawn(move || {
            let cursor = BufReader::new(stream);
            for line in cursor.lines().flatten() {
                event_tx.send(AppEvent::Server(line));
            }
        });

        // request for server state
        /*
        let request = ClientRequest::ServerQuery {
            query: query.clone(),
        };
        send_client_request(context, &request)?;
        */

        // request for server state
        let request = ClientRequest::PlayerState;
        send_client_request(context, &request)?;
    }

    loop {
        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Server(message) => {
                let server_broadcast_event: ServerBroadcastEvent = serde_json::from_str(&message)?;
                match server_broadcast_event {
                    ServerBroadcastEvent::ServerQuery { query } => {
                        println!("{}", query);
                        break;
                    }
                    ServerBroadcastEvent::PlayerState { mut state } => {
                        if !state.playlist_ref().is_empty() {
                            state.playlist_mut().set_cursor_index(Some(0));
                        }
                        context.server_state_mut().set_player(state);
                        let query = query_local(context, &query)?;
                        println!("{}", query);
                        break;
                    }
                    ServerBroadcastEvent::ServerError { msg } => {
                        println!("{}", msg);
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn query_local(context: &AppContext, query: &str) -> DiziResult<String> {
    let mut vars = HashMap::new();

    let player_state = context.server_state_ref().player_ref();

    vars.insert(
        "player_status".to_string(),
        player_state.get_player_status().to_string(),
    );
    vars.insert(
        "player_volume".to_string(),
        format!("{}", player_state.get_volume()),
    );
    vars.insert(
        "player_next".to_string(),
        format!("{}", player_state.next_enabled()),
    );
    vars.insert(
        "player_repeat".to_string(),
        format!("{}", player_state.repeat_enabled()),
    );
    vars.insert(
        "player_shuffle".to_string(),
        format!("{}", player_state.shuffle_enabled()),
    );

    if let Some(song) = player_state.get_song() {
        vars.insert("file_name".to_string(), song.file_name().to_string());
        vars.insert(
            "file_path".to_string(),
            song.file_path().to_string_lossy().to_string(),
        );
    }

    vars.insert(
        "playlist_status".to_string(),
        player_state.get_playlist_status().to_string(),
    );

    if let Some(index) = player_state.playlist_ref().get_playing_index() {
        vars.insert("playlist_index".to_string(), format!("{}", index));
    }
    vars.insert(
        "playlist_length".to_string(),
        format!("{}", player_state.playlist_ref().len()),
    );

    match strfmt(&query, &vars) {
        Ok(s) => Ok(s),
        Err(_e) => Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "Failed to process query".to_string(),
        )),
    }
}
