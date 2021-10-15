use std::path;
use std::time;

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

    // player related
    PlaylistGet,
    PlaylistAdd,
    PlaylistRemove,

    PlayerGet,
    PlayerPause,
    PlayerTogglePlay,
    PlayerToggleShuffle,
    PlayerToggleRepeat,
    PlayerToggleNext,

    PlayerVolumeUp(usize),
    PlayerVolumeDown(usize),

    PlayerPlayNext,
    PlayerPlayPrevious,

    PlayerRewind(time::Duration),
    PlayerFastForward(time::Duration),
}
