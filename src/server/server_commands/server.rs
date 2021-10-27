use std::collections::HashMap;

use strfmt::strfmt;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::context::{AppContext, QuitType};

pub fn quit_server(context: &mut AppContext) -> DiziResult<()> {
    context.quit = QuitType::Server;
    Ok(())
}

pub fn query(context: &mut AppContext, query: &str) -> DiziResult<String> {
    let mut vars = HashMap::new();

    let player_state = context.player_ref().clone_player_state();

    if let Some(song) = player_state.get_song() {
        vars.insert(
            "player_status".to_string(),
            player_state.get_player_status().to_string(),
        );
        vars.insert("file_name".to_string(), song.file_name().to_string());
        vars.insert(
            "file_path".to_string(),
            song.file_path().to_string_lossy().to_string(),
        );
    }
    match strfmt(&query, &vars) {
        Ok(s) => Ok(s),
        Err(_e) => Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "Failed to process query".to_string(),
        )),
    }
}
