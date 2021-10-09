use dizi_commands::error::DiziResult;

use crate::commands::*;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::ui::TuiBackend;

use super::{AppExecute, Command};

impl AppExecute for Command {
    fn execute(
        &self,
        context: &mut AppContext,
        backend: &mut TuiBackend,
        keymap_t: &AppKeyMapping,
    ) -> DiziResult<()> {
        match &*self {
            Self::ChangeDirectory(p) => {
                change_directory::change_directory(context, p.as_path())?;
                Ok(())
            }
            Self::CommandLine(p, s) => {
                command_line::read_and_execute(context, backend, keymap_t, p.as_str(), s.as_str())
            }

            Self::CursorMoveUp(u) => cursor_move::up(context, *u),
            Self::CursorMoveDown(u) => cursor_move::down(context, *u),
            Self::CursorMoveHome => cursor_move::home(context),
            Self::CursorMoveEnd => cursor_move::end(context),
            Self::CursorMovePageUp => cursor_move::page_up(context, backend),
            Self::CursorMovePageDown => cursor_move::page_down(context, backend),

            Self::ParentDirectory => parent_directory::parent_directory(context),

            Self::Close => quit::close(context),
            Self::Quit => quit::quit_server(context),

            Self::ReloadDirList => reload::reload_dirlist(context),

            Self::SearchGlob(pattern) => search_glob::search_glob(context, pattern.as_str()),
            Self::SearchString(pattern) => search_string::search_string(context, pattern.as_str()),
            Self::SearchSkim => search_skim::search_skim(context, backend),
            Self::SearchNext => search::search_next(context),
            Self::SearchPrev => search::search_prev(context),

            Self::SelectFiles(pattern, options) => {
                selection::select_files(context, pattern.as_str(), options)
            }

            Self::ToggleHiddenFiles => show_hidden::toggle_hidden(context),

            Self::Sort(t) => sort::set_sort(context, *t),
            Self::SortReverse => sort::toggle_reverse(context),

            Self::OpenFile => open_file::open(context),
            Self::PlayerTogglePlay => player::player_toggle_play(context),

            Self::PlayerVolumeUp(i) => player::player_volume_increase(context, *i),
            Self::PlayerVolumeDown(i) => player::player_volume_decrease(context, *i),

            /*
                        Self::PlayerToggleShuffle => {

                        }
                        Self::PlayerToggleRepeat => {

                        }
                        Self::PlayerToggleNext => {

                        }
                        Self::PlayerRewind => {

                        }
                        Self::PlayerFastForward => {

                        }
            */
            s => {
                eprintln!("Error: '{:?}' not implemented", s);
                Ok(())
            }
        }
    }
}
