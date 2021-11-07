use dizi_lib::error::DiziResult;

use crate::context::AppContext;
use crate::fs::DirList;
use crate::util::search::SearchPattern;

use super::cursor_move;

pub fn search_string_fwd(curr_list: &DirList, pattern: &str) -> Option<usize> {
    let offset = curr_list.index? + 1;
    let contents_len = curr_list.contents.len();
    for i in 0..contents_len {
        let file_name_lower = curr_list.contents[(offset + i) % contents_len]
            .file_name()
            .to_lowercase();
        if file_name_lower.contains(pattern) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}
pub fn search_string_rev(curr_list: &DirList, pattern: &str) -> Option<usize> {
    let offset = curr_list.index?;
    let contents_len = curr_list.contents.len();
    for i in (0..contents_len).rev() {
        let file_name_lower = curr_list.contents[(offset + i) % contents_len]
            .file_name()
            .to_lowercase();
        if file_name_lower.contains(pattern) {
            return Some((offset + i) % contents_len);
        }
    }
    None
}

pub fn search_string(context: &mut AppContext, pattern: &str) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let pattern = pattern.to_lowercase();

    let index = match context.curr_list_ref() {
        Some(list) => search_string_fwd(list, pattern.as_str()),
        None => None,
    };
    if let Some(index) = index {
        cursor_move::cursor_move(context, widget, index);
    }
    context.set_search_context(SearchPattern::String(pattern));
    Ok(())
}
