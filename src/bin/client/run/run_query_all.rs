use std::io::{BufRead, BufReader};
use std::thread;

use dizi::error::DiziResult;
use dizi::request::client::ClientRequest;
use dizi::response::server::ServerBroadcastEvent;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::util::request::send_client_request;

pub fn run_query_all(context: &mut AppContext) -> DiziResult {
    // server listener
    {
        let stream = context.clone_stream()?;
        let event_tx = context.clone_event_tx();

        let _ = thread::spawn(move || {
            let cursor = BufReader::new(stream);
            for line in cursor.lines().flatten() {
                let _ = event_tx.send(AppEvent::Server(line));
            }
        });

        // request for server state
        let request = ClientRequest::ServerQueryAll;
        send_client_request(context, &request)?;

        // request for server state
        let request = ClientRequest::PlayerState;
        send_client_request(context, &request)?;
    }

    loop {
        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        if let AppEvent::Server(message) = event {
            let server_broadcast_event: ServerBroadcastEvent = serde_json::from_str(&message)?;
            match server_broadcast_event {
                ServerBroadcastEvent::ServerQueryAll { mut query_items } => {
                    let mut items_sorted: Vec<(String, String)> = query_items.drain().collect();
                    items_sorted.sort();
                    for (key, val) in items_sorted {
                        println!("{} = {}", key, val);
                    }
                    break;
                }
                ServerBroadcastEvent::PlayerState { mut state } => {
                    if !state.playlist_ref().is_empty() {
                        state.playlist_mut().set_cursor_index(Some(0));
                    }
                    let mut query_items = state.query_all();
                    let mut items_sorted: Vec<(String, String)> = query_items.drain().collect();
                    items_sorted.sort();
                    for (key, val) in items_sorted {
                        println!("{} = {}", key, val);
                    }
                    break;
                }
                ServerBroadcastEvent::ServerError { msg } => {
                    println!("{}", msg);
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
