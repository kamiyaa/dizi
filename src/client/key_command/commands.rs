use std::path;

use dizi_lib::request::client::ClientRequest;

use crate::util::select::SelectOption;
use crate::util::sort_type::SortType;

#[derive(Clone, Debug)]
pub enum Command {
    Close,
    Quit,

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

    ToggleHiddenFiles,

    Request(ClientRequest),
}
