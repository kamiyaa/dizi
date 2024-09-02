use std::path;
use std::thread;

use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;

pub struct Background {}

impl Background {
    pub fn load_preview(context: &mut AppContext, p: path::PathBuf) -> thread::JoinHandle<()> {
        let event_tx = context.clone_event_tx();
        let options = context.config_ref().display_options_ref().clone();

        thread::spawn(move || {
            let path_clone = p.clone();
            let dir_res = JoshutoDirList::from_path(p, &options);
            let res = AppEvent::PreviewDir {
                path: path_clone,
                res: Box::new(dir_res),
            };
            let _ = event_tx.send(res);
        })
    }
}
