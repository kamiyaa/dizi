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

use dizi_commands::api_command::ApiCommand;
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
        match line {
            Ok(line) => {
                eprintln!("line: {}", line);
                // parse into json
                let json_res: Result<HashMap<String, String>, serde_json::Error> =
                    serde_json::from_str(&line);

                if let Ok(json_map) = json_res {
                    if let Some(s) = json_map.get("command") {
                        if let Ok(command) = ApiCommand::from_str(s) {
                            let mut context = context.lock().unwrap();
                            run_command(&mut context, command, &json_map);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {:?}", e);
            }
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
