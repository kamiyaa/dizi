use dizi_lib::error::DiziResult;

use crate::context::AppContext;
use crate::tab::JoshutoTab;
use crate::util::search::SearchPattern;

use super::cursor_move;

fn _search_exact(curr_tab: &JoshutoTab, pattern: &str) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let contents_len = curr_list.contents.len();
    for i in 0..contents_len {
        let file_name = curr_list.contents[i].file_name();
        if file_name == pattern {
            return Some(i);
        }
    }
    None
}

pub fn search_exact(context: &mut AppContext, pattern: &str) -> DiziResult<()> {
    let index = _search_exact(context.tab_context_ref().curr_tab_ref(), pattern);
    if let Some(index) = index {
        let _ = cursor_move::cursor_move(context, index);
    }
    Ok(())
}

pub fn search_string_fwd(curr_tab: &JoshutoTab, pattern: &str) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.get_index()? + 1;
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
pub fn search_string_rev(curr_tab: &JoshutoTab, pattern: &str) -> Option<usize> {
    let curr_list = curr_tab.curr_list_ref()?;

    let offset = curr_list.get_index()?;
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
    let pattern = pattern.to_lowercase();
    let index = search_string_fwd(context.tab_context_ref().curr_tab_ref(), pattern.as_str());
    if let Some(index) = index {
        let _ = cursor_move::cursor_move(context, index);
    }
    context.set_search_context(SearchPattern::String(pattern));
    Ok(())
}
