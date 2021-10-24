use globset::{GlobBuilder, GlobMatcher};

use dizi_lib::error::DiziResult;

use crate::context::AppContext;
use crate::fs::DirList;
use crate::util::search::SearchPattern;

use super::cursor_move;

pub fn search_glob_fwd(curr_list: &DirList, glob: &GlobMatcher) -> Option<usize> {
    let offset = curr_list.index? + 1;
    let contents_len = curr_list.len();
    for i in 0..contents_len {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();
        if glob.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}
pub fn search_glob_rev(curr_list: &DirList, glob: &GlobMatcher) -> Option<usize> {
    let offset = curr_list.index?;
    let contents_len = curr_list.len();
    for i in (0..contents_len).rev() {
        let file_name = curr_list.contents[(offset + i) % contents_len].file_name();
        if glob.is_match(file_name) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}

pub fn search_glob(context: &mut AppContext, pattern: &str) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let glob = GlobBuilder::new(pattern)
        .case_insensitive(true)
        .build()?
        .compile_matcher();

    let index = cursor_move::cursor_index(context, widget);
    if index.is_some() {
        let index = search_glob_fwd(context.curr_list_ref().unwrap(), &glob);
        if let Some(index) = index {
            cursor_move::cursor_move(context, widget, index);
        }
        context.set_search_context(SearchPattern::Glob(glob));
    }
    Ok(())
}
