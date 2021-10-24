use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::thread;

use strfmt::strfmt;
use termion::event::Event;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_lib::request::client::ClientRequest;
use dizi_lib::response::server::ServerBroadcastEvent;

use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::key_command::{AppExecute, CommandKeybind};
use crate::preview::preview_default;
use crate::ui::views::TuiView;
use crate::ui::TuiBackend;
use crate::util::input;
use crate::util::request::send_client_request;
use crate::util::to_string::ToString;

pub fn run_ui(
    backend: &mut TuiBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> DiziResult<()> {
    context.flush_stream();

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
        let request = ClientRequest::PlayerState;
        send_client_request(context, &request)?;
    }

    while context.quit == QuitType::DoNot {
        backend.render(TuiView::new(&context));

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Termion(Event::Mouse(_event)) => {
                context.flush_event();
            }
            AppEvent::Termion(key) => {
                if context.message_queue_ref().current_message().is_some() {
                    context.message_queue_mut().pop_front();
                }

                match keymap_t.as_ref().get(&key) {
                    None => {
                        context
                            .message_queue_mut()
                            .push_info(format!("Unmapped input: {}", key.to_string()));
                    }
                    Some(CommandKeybind::SimpleKeybind(command)) => {
                        if let Err(e) = command.execute(context, backend, &keymap_t) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {
                        let cmd = input::get_input_while_composite(backend, context, m);

                        if let Some(command) = cmd {
                            if let Err(e) = command.execute(context, backend, &keymap_t) {
                                context.message_queue_mut().push_error(e.to_string());
                            }
                        }
                    }
                }
                preview_default::load_preview(context, backend);
                context.flush_event();
            }
            AppEvent::Server(message) => {
                if let Err(err) = input::process_server_event(context, message.as_str()) {
                    context.message_queue_mut().push_error(err.to_string());
                }
            }
            event => input::process_noninteractive(event, context),
        }
    }
    Ok(())
}

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
        let request = ClientRequest::ServerQuery {
            query: query.clone(),
        };
        send_client_request(context, &request)?;

        /*
                // request for server state
                let request = ClientRequest::PlayerState;
                send_client_request(context, &request)?;
        */
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

    if let Some(song) = player_state.get_song() {
        vars.insert("file_name".to_string(), song.file_name().to_string());
        vars.insert(
            "file_path".to_string(),
            song.file_path().to_string_lossy().to_string(),
        );
    }

    match strfmt(&query, &vars) {
        Ok(s) => Ok(s),
        Err(_e) => Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "Failed to process query".to_string(),
        )),
    }
}
