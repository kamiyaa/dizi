use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread;

use dizi::error::AppResult;
use dizi::request::client::ClientRequest;
use dizi::response::server::ServerBroadcastEvent;
use dizi::utils;

use crate::events::{AppEvent, AppEventSender, ClientMessageSender};

#[derive(Clone, Debug)]
pub enum ClientMessage {
    Client(String),
    Server(Box<ServerBroadcastEvent>),
}

pub fn handle_client(
    uuid: uuid::Uuid,
    mut stream: UnixStream,
    app_event_tx: AppEventSender,
    event_tx: ClientMessageSender,
    event_rx: mpsc::Receiver<ClientMessage>,
) -> AppResult {
    // the server broadcasts to this client over another clone of `event_tx`
    let uuid_string = uuid.to_string();

    // listen for requests sent by client
    let event_tx_clone = event_tx;
    let stream_clone = stream.try_clone().expect("Failed to clone UnixStream");
    let _ = thread::spawn(move || {
        let cursor = BufReader::new(stream_clone);
        // keep listening for client requests
        for line in cursor.lines().map_while(Result::ok) {
            if event_tx_clone.send(ClientMessage::Client(line)).is_err() {
                return;
            }
        }

        let response = ClientRequest::ClientLeave {
            uuid: uuid.to_string(),
        };
        let json = serde_json::to_string(&response).expect("Failed to serialize ClientRequest");
        let _ = event_tx_clone.send(ClientMessage::Client(json));
    });

    // process events
    while let Ok(event) = event_rx.recv() {
        match event {
            ClientMessage::Server(event) => {
                process_server_event(&mut stream, &event)?;
            }
            ClientMessage::Client(line) => {
                if line.is_empty() {
                    continue;
                }
                forward_client_request(&app_event_tx, &uuid_string, &line)?;
            }
        }
    }
    Ok(())
}

/// Forwards client requests to the server's main loop
pub fn forward_client_request(app_event_tx: &AppEventSender, uuid: &str, line: &str) -> AppResult {
    let request: ClientRequest = serde_json::from_str(line)?;
    app_event_tx.send(AppEvent::Client {
        uuid: uuid.to_string(),
        request,
    })?;
    Ok(())
}

pub fn process_server_event(stream: &mut UnixStream, event: &ServerBroadcastEvent) -> AppResult {
    let json = serde_json::to_string(&event)?;
    stream.write_all(json.as_bytes())?;
    utils::flush(stream)?;
    Ok(())
}
