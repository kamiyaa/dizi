use std::fs;
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

use dizi_lib::error::DiziResult;

use crate::client;
use crate::config::default::AppConfig;
use crate::context::AppContext;
use crate::events::{AppEvent, ClientRequest, Events, ServerBroadcastEvent, ServerEvent};
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
    let mut context = AppContext::new(config);

    let listener = setup_socket(context.config_ref())?;
    {
        // thread for listening to new client connections
        let client_tx2 = context.events.client_request_sender().clone();
        thread::spawn(|| client::listen_for_clients(listener, client_tx2));
    }

    loop {
        let event = match context.events.next() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        match event {
            AppEvent::Client(client_event) => match client_event {
                ClientRequest::Quit => {
                    break;
                }
                ClientRequest::NewClient(stream) => {
                    let client_tx2 = context.events.client_request_sender().clone();
                    let (server_tx, server_rx) = mpsc::channel();

                    thread::spawn(|| client::handle_client(stream, client_tx2, server_rx));
                    context.events.add_broadcast_listener(server_tx);
                }
                event => {
                    let _ = run_command(&mut context, event);
                }
            },
            AppEvent::Server(event) => {
                process_server_event(&mut context, event);
            }
        }
    }

    Ok(())
}

pub fn process_server_event(context: &mut AppContext, event: ServerEvent) {
    match event {
        ServerEvent::PlayerProgressUpdate(t) => {
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerProgressUpdate(t));
        }
        ServerEvent::PlayerDone => {
            // TODO: Play next song
            eprintln!("End of playlist!");
        }
    }
}
