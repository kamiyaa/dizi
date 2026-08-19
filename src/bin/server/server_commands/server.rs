use dizi::error::AppResult;

use crate::{
    context::{AppContext, QuitType},
    traits::AudioPlayer,
};

pub fn quit_server(context: &mut AppContext) -> AppResult {
    context.quit = QuitType::Server;
    Ok(())
}

pub fn query(context: &mut AppContext, query: &str) -> AppResult<String> {
    let player_state = context.player.player_state();
    let res = player_state.query(query)?;
    Ok(res)
}
