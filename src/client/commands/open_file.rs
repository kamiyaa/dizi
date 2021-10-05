use std::io;
use std::io::{Read, Write};
use std::path;

use dizi_commands::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_commands::structs::PlayerPlay;
use dizi_commands::api_command::ApiCommand;

use crate::context::AppContext;
use crate::fs::DirEntry;
use crate::ui::views::TuiTextField;
use crate::ui::TuiBackend;

use super::change_directory;

pub const NEWLINE: &[u8] = &['\n' as u8];

pub fn open(context: &mut AppContext, backend: &mut TuiBackend) -> DiziResult<()> {
    let config = context.config_ref();

    if let Some(entry) = context
        .curr_list_ref()
        .and_then(|s| s.curr_entry_ref())
    {
        if entry.file_path().is_dir() {
            let path = entry.file_path().to_path_buf();
            change_directory::cd(path.as_path(), context)?;
        } else {
            play_file(context, backend, entry.file_path().to_path_buf());
        }
    }
    Ok(())
}

pub fn play_file(context: &mut AppContext, backend: &mut TuiBackend, path: path::PathBuf) -> DiziResult<()> {
    let request = PlayerPlay {
        command: ApiCommand::PlayerPlay.to_str().to_string(),
        path,
    };

    eprintln!("{:?}", request);

    let json = serde_json::to_string(&request).unwrap();
    eprintln!("{:?}", json);

    let res = context.stream.write(json.as_bytes());
    eprintln!("{:?}", res);
    let res = context.stream.write(NEWLINE);
    Ok(())
}
