use std::thread;
use std::time::SystemTime;
use std::{io, path};

use crate::config::option::DisplayOption;
use crate::context::AppState;
use crate::event::AppEvent;
use crate::fs::AppDirectoryList;

#[derive(Clone, Debug)]
pub enum PreviewDirState {
    Loading,
    Error,
}

pub struct Background {}

impl Background {
    /// Spawns a background thread that reads `dir_path` and posts an [`AppEvent::PreviewDir`]
    /// with the result. If the current tab already has a listing for `dir_path` that isn't
    /// marked stale, the thread first checks the directory's mtime and skips the re-read when
    /// it hasn't changed. Even that check stays off the main thread, since a single `stat` can
    /// block for seconds on a spun-down disk.
    pub fn refresh(context: &mut AppState, dir_path: path::PathBuf) -> thread::JoinHandle<()> {
        let cached_modified = context
            .tab_state
            .curr_tab_ref()
            .history
            .get(dir_path.as_path())
            .filter(|list| !list.need_update)
            .map(|list| list.metadata.modified());
        Self::spawn(context, dir_path, cached_modified)
    }

    fn spawn(
        context: &mut AppState,
        dir_path: path::PathBuf,
        cached_modified: Option<SystemTime>,
    ) -> thread::JoinHandle<()> {
        let event_tx = context.clone_event_tx();
        let options = context.config.client.display_options.clone();

        context
            .tab_state
            .curr_tab_mut()
            .history_metadata
            .insert(dir_path.clone(), PreviewDirState::Loading);

        thread::spawn(move || {
            let path_clone = dir_path.clone();
            let dir_res = read_if_changed(dir_path, cached_modified, &options);
            let res = AppEvent::PreviewDir {
                path: path_clone,
                res: Box::new(dir_res),
            };
            let _ = event_tx.send(res);
        })
    }
}

/// Reads `dir_path`, unless `cached_modified` is given and the directory hasn't been modified
/// since then, in which case returns `Ok(None)`.
fn read_if_changed(
    dir_path: path::PathBuf,
    cached_modified: Option<SystemTime>,
    options: &DisplayOption,
) -> io::Result<Option<AppDirectoryList>> {
    let unchanged = cached_modified.is_some_and(|cached| {
        // follow symlinks, like `AppFileMetadata::from` does for the cached listing;
        // if the stat fails, keep showing the cached listing
        std::fs::metadata(&dir_path)
            .and_then(|m| m.modified())
            .map(|m| m <= cached)
            .unwrap_or(true)
    });
    if unchanged {
        return Ok(None);
    }
    AppDirectoryList::from_path(dir_path, options).map(Some)
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use crate::config::option::DisplayOption;

    use super::read_if_changed;

    fn temp_dir() -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("dizi-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(dir.join("a"), "").unwrap();
        dir
    }

    #[test]
    fn unchanged_directory_is_not_reread() {
        let dir = temp_dir();
        let far_future = SystemTime::now() + Duration::from_secs(3600);
        let res = read_if_changed(dir.clone(), Some(far_future), &DisplayOption::default());
        assert!(matches!(res, Ok(None)));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn changed_or_uncached_directory_is_read() {
        let dir = temp_dir();
        for cached in [None, Some(SystemTime::UNIX_EPOCH)] {
            let res = read_if_changed(dir.clone(), cached, &DisplayOption::default());
            assert_eq!(res.unwrap().unwrap().contents.len(), 1);
        }
        std::fs::remove_dir_all(dir).unwrap();
    }
}
