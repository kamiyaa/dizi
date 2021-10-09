use dizi_commands::constants::*;

use super::constants::*;
use super::{AppCommand, Command};

impl AppCommand for Command {
    fn command(&self) -> &'static str {
        match self {
            Self::Close => CMD_CLOSE,
            Self::Quit => CMD_QUIT,

            Self::ChangeDirectory(_) => CMD_CHANGE_DIRECTORY,
            Self::CommandLine(_, _) => CMD_COMMAND_LINE,

            Self::CursorMoveUp(_) => CMD_CURSOR_MOVE_UP,
            Self::CursorMoveDown(_) => CMD_CURSOR_MOVE_DOWN,
            Self::CursorMoveHome => CMD_CURSOR_MOVE_HOME,
            Self::CursorMoveEnd => CMD_CURSOR_MOVE_END,
            Self::CursorMovePageUp => CMD_CURSOR_MOVE_PAGEUP,
            Self::CursorMovePageDown => CMD_CURSOR_MOVE_PAGEDOWN,

            Self::OpenFile => CMD_OPEN_FILE,
            Self::ParentDirectory => CMD_PARENT_DIRECTORY,

            Self::ReloadDirList => CMD_RELOAD_DIRECTORY_LIST,

            Self::SearchString(_) => CMD_SEARCH_STRING,
            Self::SearchGlob(_) => CMD_SEARCH_GLOB,
            Self::SearchSkim => CMD_SEARCH_SKIM,
            Self::SearchNext => CMD_SEARCH_NEXT,
            Self::SearchPrev => CMD_SEARCH_PREV,

            Self::SelectFiles(_, _) => CMD_SELECT_FILES,

            Self::Sort(_) => CMD_SORT,
            Self::SortReverse => CMD_SORT_REVERSE,

            Self::ToggleHiddenFiles => CMD_TOGGLE_HIDDEN,

            Self::PlaylistGet => API_PLAYLIST_GET,
            Self::PlaylistAdd => API_PLAYLIST_ADD,
            Self::PlaylistRemove => API_PLAYLIST_REMOVE,

            Self::PlayerGet => API_PLAYER_GET,
            Self::PlayerPlay => API_PLAYER_PLAY,
            Self::PlayerPause => API_PLAYER_PAUSE,
            Self::PlayerTogglePlay => API_PLAYER_TOGGLE_PLAY,
            Self::PlayerToggleShuffle => API_PLAYER_TOGGLE_SHUFFLE,
            Self::PlayerToggleRepeat => API_PLAYER_TOGGLE_REPEAT,
            Self::PlayerToggleNext => API_PLAYER_TOGGLE_NEXT,

            Self::PlayerVolumeUp(_) => API_PLAYER_VOLUME_UP,
            Self::PlayerVolumeDown(_) => API_PLAYER_VOLUME_DOWN,

            Self::PlayerRewind(_) => API_PLAYER_REWIND,
            Self::PlayerFastForward(_) => API_PLAYER_FAST_FORWARD,
        }
    }
}
