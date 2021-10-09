use std::fs;
use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::os::unix::net::UnixListener;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use dizi_lib::error::DiziResult;

use crate::client;
use crate::command::run_command;
use crate::config::default::AppConfig;
use crate::context::AppContext;
use crate::events::{AppEvent, Events};

pub fn setup_socket(config: &AppConfig) -> DiziResult<UnixListener> {
    let socket = Path::new(config.server_ref().socket.as_path());
    if socket.exists() {
        fs::remove_file(&socket)?;
    }
    let stream = UnixListener::bind(&socket)?;
    Ok(stream)
}

pub fn serve(config: AppConfig) -> DiziResult<()> {
    let events: Events = Events::new();
    let event_tx_clone = events.event_tx.clone();
    let context = AppContext::new(config, event_tx_clone)?;

    let listener = setup_socket(context.config_ref())?;

    let context_arc = Arc::new(Mutex::new(context));

    {
        let event_tx_clone = events.event_tx.clone();
        thread::spawn(|| client::listen_for_clients(listener, event_tx_clone));
    }

    loop {
        let event = match events.next() {
            Ok(event) => event,
            Err(_) => return Ok(()),
        };

        match event {
            AppEvent::NewClient(stream) => {
                let context_arc_clone = context_arc.clone();
                thread::spawn(|| client::handle_client(stream, context_arc_clone));
            }
            AppEvent::Quit => {
                break;
            }
            s => {
                eprintln!("Unimplemented! {:?}", s);
            }
        }
    }

    Ok(())
}
