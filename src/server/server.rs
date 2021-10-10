use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;

use crate::client;
use crate::config::default::AppConfig;
use crate::context::AppContext;
use crate::events::{ClientEvent, Events};
use crate::server_command::run_command;

pub fn setup_socket(config: &AppConfig) -> DiziResult<UnixListener> {
    let socket = Path::new(config.server_ref().socket.as_path());
    if socket.exists() {
        fs::remove_file(&socket)?;
    }
    let stream = UnixListener::bind(&socket)?;
    Ok(stream)
}

pub fn serve(config: AppConfig) -> DiziResult<()> {
    let mut events = Events::new();
    let mut context = AppContext::new(config);

    let listener = setup_socket(context.config_ref())?;
    {
        // thread for listening to new client connections
        let client_tx2 = events.client_tx.clone();
        thread::spawn(|| client::listen_for_clients(listener, client_tx2));
    }

    /*
        {
            // thread for listening to new client connections
            let client_tx2 = events.client_tx.clone();
            let player = context.player_context_ref().player_clone();
            thread::spawn(move || {
                let duration = std::time::Duration::from_millis(1000);
                loop {
                    thread::sleep(duration);
                    if player.lock().unwrap().is_paused() {
                        continue;
                    }
                    while !player.lock().unwrap().is_paused() {
                        thread::sleep(duration);
                    }
                    client_tx2.send(ClientEvent::PlayerNextSong);
                }
            });
        }
    */

    loop {
        let event = match events.next() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        match event {
            ClientEvent::Quit => {
                break;
            }
            ClientEvent::NewClient(stream) => {
                let client_tx2 = events.client_tx.clone();
                let (server_tx, server_rx) = mpsc::channel();

                thread::spawn(|| client::handle_client(stream, client_tx2, server_rx));
                events.server_listeners.push(server_tx);
            }
            event => {
                let _ = run_command(&mut context, &mut events, event);
            }
        }
    }

    Ok(())
}
