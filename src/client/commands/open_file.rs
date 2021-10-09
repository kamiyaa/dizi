use dizi_lib::error::DiziResult;

use crate::commands::player;
use crate::context::AppContext;
use crate::ui::TuiBackend;

use super::change_directory;

pub fn open(context: &mut AppContext) -> DiziResult<()> {
    if let Some(entry) = context.curr_list_ref().and_then(|s| s.curr_entry_ref()) {
        if entry.file_path().is_dir() {
            let path = entry.file_path().to_path_buf();
            change_directory::cd(path.as_path(), context)?;
        } else {
            player::player_play(context, entry.file_path().to_path_buf())?;
        }
    }
    Ok(())
}
