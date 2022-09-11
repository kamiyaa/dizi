use rustyline::completion::Pair;

pub const CMD_COMMAND_LINE: &str = ":";

macro_rules! cmd_constants {
    ($( ($cmd_name:ident, $cmd_value:literal), )*) => {
        $(
            pub const $cmd_name: &str = $cmd_value;
        )*

        pub fn commands() -> Vec<&'static str> {
            vec![$($cmd_value,)*]
        }
    };
}

cmd_constants![
    (CMD_CLOSE, "close"),
    (CMD_CHANGE_DIRECTORY, "cd"),
    (CMD_CURSOR_MOVE_UP, "cursor_move_up"),
    (CMD_CURSOR_MOVE_DOWN, "cursor_move_down"),
    (CMD_CURSOR_MOVE_HOME, "cursor_move_home"),
    (CMD_CURSOR_MOVE_END, "cursor_move_end"),
    (CMD_CURSOR_MOVE_PAGEUP, "cursor_move_page_up"),
    (CMD_CURSOR_MOVE_PAGEDOWN, "cursor_move_page_down"),
    (CMD_GO_TO_PLAYING, "go_to_playing"),
    (CMD_OPEN_FILE, "open"),
    (CMD_PARENT_DIRECTORY, "cd .."),
    (CMD_RELOAD_DIRECTORY_LIST, "reload_dirlist"),
    (CMD_SEARCH_STRING, "search"),
    (CMD_SEARCH_GLOB, "search_glob"),
    (CMD_SEARCH_SKIM, "search_skim"),
    (CMD_SEARCH_NEXT, "search_next"),
    (CMD_SEARCH_PREV, "search_prev"),
    (CMD_SELECT_FILES, "select"),
    (CMD_SERVER_REQUEST, "server_request"),
    (CMD_SORT, "sort"),
    (CMD_SORT_REVERSE, "sort reverse"),
    (CMD_TOGGLE_HIDDEN, "toggle_hidden"),
    (CMD_TOGGLE_VIEW, "toggle_view"),
];

pub fn complete_command(partial_command: &str) -> Vec<Pair> {
    commands()
        .into_iter()
        .filter(|command| command.starts_with(partial_command))
        .map(|command| Pair {
            display: command.to_string(),
            replacement: command.to_string(),
        })
        .collect()
}
