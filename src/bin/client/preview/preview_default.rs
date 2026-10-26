use std::path;

use crate::context::AppState;
use crate::fs::AppFileMetadata;
use crate::preview::preview_dir;
use crate::ui::AppBackend;

pub fn load_preview_path(
    context: &mut AppState,
    _backend: &mut AppBackend,
    p: path::PathBuf,
    metadata: AppFileMetadata,
) {
    if metadata.is_dir() {
        let tab = context.tab_state.curr_tab_ref();
        // only if there isn't already a loading thread or a failed load; whether a cached
        // listing is out of date is checked by the loading thread, to keep IO off this one
        if !tab.history_metadata.contains_key(p.as_path()) {
            preview_dir::Background::refresh(context, p);
        }
    }
}

pub fn load_preview(context: &mut AppState, backend: &mut AppBackend) {
    let mut load_list = Vec::with_capacity(2);

    match context.tab_state.curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            if let Some(index) = curr_list.get_index() {
                let entry = &curr_list.contents[index];
                load_list.push((entry.file_path().to_path_buf(), entry.metadata.clone()));
            }
        }
        None => {
            let cwd = context.tab_state.curr_tab_mut().get_cwd();
            if let Ok(metadata) = AppFileMetadata::from(cwd) {
                load_list.push((cwd.to_path_buf(), metadata));
            }
        }
    }

    for (path, metadata) in load_list {
        load_preview_path(context, backend, path, metadata);
    }
}
