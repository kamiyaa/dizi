use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;
use dizi_lib::response;
use dizi_lib::utils;

use crate::client_command::run_command;
use crate::events::{
    ClientRequest, ClientRequestSender, ServerBroadcastEvent, ServerBroadcastEventReceiver,
};

#[derive(Clone, Debug)]
pub enum ClientMessage {
    Client(String),
    Server(ServerBroadcastEvent),
}

pub fn handle_client(
    mut stream: UnixStream,
    client_request_tx: ClientRequestSender,
    server_event_rx: ServerBroadcastEventReceiver,
) {
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

    // listen for client requests
    let event_tx_clone = event_tx.clone();
    let stream_clone = stream.try_clone().unwrap();
    let _ = thread::spawn(move || {
        let cursor = BufReader::new(stream_clone);
        for line in cursor.lines() {
            if let Ok(line) = line {
                if event_tx_clone.send(ClientMessage::Client(line)).is_err() {
                    return;
                }
            }
        }
    });

    // process events
    while let Ok(event) = event_rx.recv() {
        match event {
            ClientMessage::Server(event) => {
                let _ = process_server_event(&mut stream, event);
            }
            ClientMessage::Client(line) => {
                let _ = run_command(&client_request_tx, &line);
            }
        }
    }
}

pub fn listen_for_clients(listener: UnixListener, event_tx: ClientRequestSender) -> DiziResult<()> {
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            event_tx.send(ClientRequest::NewClient(stream));
        }
    }
    Ok(())
}

macro_rules! server_process_stub {
    ($stream:ident, $enum_variant:ident) => {
        let response = response::$enum_variant::new();
        let json = serde_json::to_string(&response).unwrap();

        $stream.write(json.as_bytes())?;
        utils::flush($stream)?;
    };
}

pub fn process_server_event(
    stream: &mut UnixStream,
    event: ServerBroadcastEvent,
) -> DiziResult<()> {
    eprintln!("event from server: {:?}", event);
    match event {
        ServerBroadcastEvent::Quit => {}
        ServerBroadcastEvent::PlayerPlay(song) => {
            let response = response::PlayerPlay::new(song);
            let json = serde_json::to_string(&response).unwrap();

            stream.write(json.as_bytes())?;
            utils::flush(stream)?;
        }
        ServerBroadcastEvent::PlayerVolumeUpdate(volume) => {
            let response = response::PlayerVolumeUpdate::new(volume);
            let json = serde_json::to_string(&response).unwrap();

            stream.write(json.as_bytes())?;
            utils::flush(stream)?;
        }
        ServerBroadcastEvent::PlayerProgressUpdate(duration) => {
            let response = response::PlayerProgressUpdate::new(duration);
            let json = serde_json::to_string(&response).unwrap();

            stream.write(json.as_bytes())?;
            utils::flush(stream)?;
        }
        ServerBroadcastEvent::PlayerPause => {
            server_process_stub!(stream, PlayerPause);
        }
        ServerBroadcastEvent::PlayerResume => {
            server_process_stub!(stream, PlayerResume);
        }
        ServerBroadcastEvent::PlayerRepeatOn => {
            server_process_stub!(stream, PlayerRepeatOn);
        }
        ServerBroadcastEvent::PlayerRepeatOff => {
            server_process_stub!(stream, PlayerRepeatOff);
        }
        ServerBroadcastEvent::PlayerShuffleOn => {
            server_process_stub!(stream, PlayerShuffleOn);
        }
        ServerBroadcastEvent::PlayerShuffleOff => {
            server_process_stub!(stream, PlayerShuffleOff);
        }
        s => {
            eprintln!("Not Implemented! {:?}", s);
        }
    }
    Ok(())
}
