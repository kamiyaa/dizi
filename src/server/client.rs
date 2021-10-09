use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use dizi_lib::error::DiziResult;

use crate::command::run_command;
use crate::context::AppContext;
use crate::events::AppEvent;

pub fn handle_client(stream: UnixStream, context: Arc<Mutex<AppContext>>) {
    let cursor = BufReader::new(stream);
    for line in cursor.lines() {
        if let Ok(line) = line {
            let mut context = context.lock().unwrap();
            let res = run_command(&mut context, &line);
            eprintln!("res: {:?}", res);
        }
    }
}

pub fn listen_for_clients(
    listener: UnixListener,
    event_tx: mpsc::Sender<AppEvent>,
) -> DiziResult<()> {
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            event_tx.send(AppEvent::NewClient(stream));
        }
    }
    Ok(())
}
