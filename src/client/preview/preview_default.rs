use std::path;

use crate::context::AppContext;
use crate::fs::JoshutoMetadata;
use crate::preview::preview_dir;
use crate::ui::AppBackend;

pub fn load_preview_path(
    context: &mut AppContext,
    _backend: &mut AppBackend,
    p: path::PathBuf,
    metadata: JoshutoMetadata,
) {
    if metadata.is_dir() {
        let need_to_load = context
            .tab_context_ref()
            .curr_tab_ref()
            .history_ref()
            .get(p.as_path())
            .map(|e| e.need_update())
            .unwrap_or(true);

        if need_to_load {
            preview_dir::Background::load_preview(context, p);
        }
    }
}

pub fn load_preview(context: &mut AppContext, backend: &mut AppBackend) {
    let mut load_list = Vec::with_capacity(2);

    match context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        Some(curr_list) => {
            if let Some(index) = curr_list.get_index() {
                let entry = &curr_list.contents[index];
                load_list.push((entry.file_path().to_path_buf(), entry.metadata.clone()));
            }
        }
        None => {
            let cwd = context.tab_context_mut().curr_tab_mut().cwd();
            if let Ok(metadata) = JoshutoMetadata::from(cwd) {
                load_list.push((cwd.to_path_buf(), metadata));
            }
        }
    }

    for (path, metadata) in load_list {
        load_preview_path(context, backend, path, metadata);
    }
}
