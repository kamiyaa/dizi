use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;
use dizi_lib::response;
use dizi_lib::utils;

use crate::client_command::run_command;
use crate::events::{ClientEvent, ClientEventSender, ServerEvent, ServerEventReceiver};

#[derive(Clone, Debug)]
pub enum ClientMessage {
    Client(String),
    Server(ServerEvent),
}

pub fn handle_client(
    mut stream: UnixStream,
    server_req: ClientEventSender,
    server_res: ServerEventReceiver,
) {
    let (event_tx, event_rx) = mpsc::channel();

    let event_tx_clone = event_tx.clone();
    let _ = thread::spawn(move || {
        while let Ok(server_event) = server_res.recv() {
            if event_tx_clone
                .send(ClientMessage::Server(server_event))
                .is_err()
            {
                return;
            }
        }
    });

    let event_tx_clone = event_tx.clone();
    let client_listen_thread = event_tx.clone();
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

    while let Ok(event) = event_rx.recv() {
        match event {
            ClientMessage::Server(event) => {
                let _ = process_server_event(&mut stream, event);
            }
            ClientMessage::Client(line) => {
                let _ = run_command(&server_req, &line);
            }
        }
    }
}

pub fn listen_for_clients(listener: UnixListener, event_tx: ClientEventSender) -> DiziResult<()> {
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            event_tx.send(ClientEvent::NewClient(stream));
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

pub fn process_server_event(stream: &mut UnixStream, event: ServerEvent) -> DiziResult<()> {
    eprintln!("event from server: {:?}", event);
    match event {
        ServerEvent::Quit => {}
        ServerEvent::PlayerPlay(song) => {
            let response = response::PlayerPlay::new(song);
            let json = serde_json::to_string(&response).unwrap();

            stream.write(json.as_bytes())?;
            utils::flush(stream)?;
        }
        ServerEvent::PlayerVolumeUpdate(volume) => {
            let response = response::PlayerVolumeUpdate::new(volume);
            let json = serde_json::to_string(&response).unwrap();

            stream.write(json.as_bytes())?;
            utils::flush(stream)?;
        }
        ServerEvent::PlayerDurationLeft(usize) => {}
        ServerEvent::PlayerPause => {
            server_process_stub!(stream, PlayerPause);
        }
        ServerEvent::PlayerResume => {
            server_process_stub!(stream, PlayerResume);
        }
        ServerEvent::PlayerRepeatOn => {
            server_process_stub!(stream, PlayerRepeatOn);
        }
        ServerEvent::PlayerRepeatOff => {
            server_process_stub!(stream, PlayerRepeatOff);
        }
        ServerEvent::PlayerShuffleOn => {
            server_process_stub!(stream, PlayerShuffleOn);
        }
        ServerEvent::PlayerShuffleOff => {
            server_process_stub!(stream, PlayerShuffleOff);
        }
        s => {
            eprintln!("Not Implemented! {:?}", s);
        }
    }
    Ok(())
}
