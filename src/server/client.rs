use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;
use dizi_lib::request::client::ClientRequest;
use dizi_lib::response::server::ServerBroadcastEvent;
use dizi_lib::utils;

use crate::events::{ClientRequestSender, ServerBroadcastEventReceiver};

#[derive(Clone, Debug)]
pub enum ClientMessage {
    Client(String),
    Server(ServerBroadcastEvent),
}

pub fn handle_client(
    uuid: uuid::Uuid,
    mut stream: UnixStream,
    client_request_tx: ClientRequestSender,
    server_event_rx: ServerBroadcastEventReceiver,
) -> DiziResult {
    let (event_tx, event_rx) = mpsc::channel();

    // listen for server events
    let event_tx_clone = event_tx.clone();
    let _ = thread::spawn(move || {
        while let Ok(server_event) = server_event_rx.recv() {
            if event_tx_clone
                .send(ClientMessage::Server(server_event))
                .is_err()
            {
                return;
            }
        }
    });

    let uuid_string = uuid.to_string();

    // listen for client requests
    let event_tx_clone = event_tx;
    let stream_clone = stream.try_clone().unwrap();
    let _ = thread::spawn(move || {
        let cursor = BufReader::new(stream_clone);
        for line in cursor.lines().flatten() {
            if event_tx_clone.send(ClientMessage::Client(line)).is_err() {
                return;
            }
        }

        let response = ClientRequest::ClientLeave {
            uuid: uuid.to_string(),
        };
        let json = serde_json::to_string(&response).unwrap();
        event_tx_clone.send(ClientMessage::Client(json));
    });

    // process events
    while let Ok(event) = event_rx.recv() {
        match event {
            ClientMessage::Server(event) => {
                process_server_event(&mut stream, event)?;
            }
            ClientMessage::Client(line) => {
                if line.is_empty() {
                    continue;
                }
                forward_client_request(&client_request_tx, &uuid_string, &line);
            }
        }
    }
    Ok(())
}

pub fn forward_client_request(
    client_request_tx: &ClientRequestSender,
    uuid: &str,
    line: &str,
) -> DiziResult {
    let request: ClientRequest = serde_json::from_str(line)?;
    client_request_tx.send((uuid.to_string(), request))?;
    Ok(())
}

pub fn process_server_event(stream: &mut UnixStream, event: ServerBroadcastEvent) -> DiziResult {
    let response = event;
    let json = serde_json::to_string(&response)?;

    stream.write_all(json.as_bytes())?;
    utils::flush(stream)?;
    Ok(())
}
