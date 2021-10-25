use std::path;

use dizi_lib::request::client::ClientRequest;

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

    OpenFile,
    ParentDirectory,

    ReloadDirList,

    SearchGlob(String),
    SearchString(String),
    SearchSkim,
    SearchNext,
    SearchPrev,

    SelectFiles(String, SelectOption),

    Sort(SortType),
    SortReverse,

    ToggleView,
    ToggleHiddenFiles,

    Request(ClientRequest),
}
