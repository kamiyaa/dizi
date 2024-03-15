use std::path;

use dizi::request::client::ClientRequest;

use crate::config::option::SelectOption;
use crate::config::option::SortType;

#[derive(Clone, Debug)]
pub enum Command {
    Close,

    ChangeDirectory(path::PathBuf),
    CommandLine(String, String),

    CursorMoveUp(usize),
    CursorMoveDown(usize),
    CursorMoveHome,
    CursorMoveEnd,
    CursorMovePageUp,
    CursorMovePageDown,

    GoToPlaying,

    OpenFile,
    ParentDirectory,

    ReloadDirList,

    SearchGlob(String),
    SearchString(String),
    SearchSkim,
    SearchNext,
    SearchPrev,

    ServerRequest(ClientRequest),

    SelectFiles(String, SelectOption),

    Sort(SortType),
    SortReverse,

    ToggleView,
    ToggleHiddenFiles,
}
