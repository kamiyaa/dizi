use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;

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
            ClientMessage::Server(event) => process_server_event(&mut stream, event),
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

pub fn process_server_event(stream: &mut UnixStream, event: ServerEvent) {
    match event {
        ServerEvent::Quit => {}
        ServerEvent::PlayerPlay(Song) => {}
        ServerEvent::PlayerPause => {}
        ServerEvent::PlayerResume => {}
        ServerEvent::PlayerRepeatOn => {}
        ServerEvent::PlayerRepeatOff => {}
        ServerEvent::PlayerShuffleOn => {}
        ServerEvent::PlayerShuffleOff => {}
        ServerEvent::PlayerVolumeUpdate(usize) => {}
        ServerEvent::PlayerDurationLeft(usize) => {}
    }
}
