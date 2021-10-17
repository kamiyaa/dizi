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

            Self::Request(request) => "",
        }
    }
}
