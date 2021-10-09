use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Cursor, Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::str;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

use lazy_static::lazy_static;

use rodio::{OutputStream, OutputStreamHandle};

use dizi_commands::error::DiziResult;

use crate::audio::Playlist;
use crate::command::run_command;
use crate::config::default::AppConfig;
use crate::context::{AppContext, PlayerContext};

/*
lazy_static! {
    pub static ref OUTPUT_STREAM: Mutex<(OutputStream, OutputStreamHandle)> = Mutex::new(OutputStream::try_default()?);
}
*/

pub fn setup_socket(config: &AppConfig) -> DiziResult<UnixListener> {
    let socket = Path::new(config.server_ref().socket.as_path());

    if socket.exists() {
        fs::remove_file(&socket)?;
    }

    let stream = UnixListener::bind(&socket)?;
    Ok(stream)
}

pub fn handle_client(stream: UnixStream, context: Arc<Mutex<AppContext>>) {
    let cursor = BufReader::new(stream);
    for line in cursor.lines() {
        if let Ok(line) = line {
            let mut context = context.lock().unwrap();
            run_command(&mut context, &line);
        }
    }
}

pub fn serve(config: AppConfig) -> DiziResult<()> {
    let context = AppContext::new(config)?;

    let listener = setup_socket(context.config_ref())?;

    let context_arc = Arc::new(Mutex::new(context));

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let context_arc_clone = context_arc.clone();

            thread::spawn(|| handle_client(stream, context_arc_clone));
        }
    }
    Ok(())
}
