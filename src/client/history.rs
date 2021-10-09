use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::fs::{DirEntry, DirList, Metadata};
use crate::util::display_option::DisplayOption;

pub trait DirectoryHistory {
    fn populate_to_root(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn create_or_soft_update(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn create_or_reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn depreciate_all_entries(&mut self);

    fn depreciate_entry(&mut self, path: &Path);
}

pub type History = HashMap<PathBuf, DirList>;

impl DirectoryHistory for History {
    fn populate_to_root(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let mut dirlists = Vec::new();

        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            if self.contains_key(curr) {
                let mut new_dirlist = create_dirlist_with_history(self, curr, options)?;
                if let Some(ancestor) = prev.as_ref() {
                    if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                        new_dirlist.index = Some(i);
                    }
                }
                dirlists.push(new_dirlist);
            } else {
                let mut new_dirlist = DirList::from_path(curr.to_path_buf().clone(), options)?;
                if let Some(ancestor) = prev.as_ref() {
                    if let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor) {
                        new_dirlist.index = Some(i);
                    }
                }
                dirlists.push(new_dirlist);
            }
            prev = Some(curr);
        }
        for dirlist in dirlists {
            self.insert(dirlist.file_path().to_path_buf(), dirlist);
        }
        Ok(())
    }

    fn create_or_soft_update(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let (contains_key, need_update) = if let Some(dirlist) = self.get(path) {
            (true, dirlist.need_update())
        } else {
            (false, true)
        };
        if need_update {
            let dirlist = if contains_key {
                create_dirlist_with_history(self, path, options)?
            } else {
                DirList::from_path(path.to_path_buf(), options)?
            };
            self.insert(path.to_path_buf(), dirlist);
        }
        Ok(())
    }

    fn create_or_reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let dirlist = if self.contains_key(path) {
            create_dirlist_with_history(self, path, options)?
        } else {
            DirList::from_path(path.to_path_buf(), options)?
        };
        self.insert(path.to_path_buf(), dirlist);
        Ok(())
    }

    fn reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let dirlist = create_dirlist_with_history(self, path, options)?;
        self.insert(path.to_path_buf(), dirlist);
        Ok(())
    }

    fn depreciate_all_entries(&mut self) {
        self.iter_mut().for_each(|(_, v)| v.depreciate());
    }

    fn depreciate_entry(&mut self, path: &Path) {
        if let Some(v) = self.get_mut(path) {
            v.depreciate();
        }
    }
}

fn get_index_of_value(arr: &[DirEntry], val: &Path) -> Option<usize> {
    arr.iter().enumerate().find_map(|(i, dir)| {
        if dir.file_path() == val {
            Some(i)
        } else {
            None
        }
    })
}

pub fn create_dirlist_with_history(
    history: &History,
    path: &Path,
    options: &DisplayOption,
) -> io::Result<DirList> {
    let filter_func = options.filter_func();
    let mut contents = read_directory(path, filter_func, options)?;

    let sort_options = options.sort_options_ref();
    contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

    let contents_len = contents.len();
    let index: Option<usize> = if contents_len == 0 {
        None
    } else {
        match history.get(path) {
            Some(dirlist) => match dirlist.index {
                Some(i) if i >= contents_len => Some(contents_len - 1),
                Some(i) => {
                    let entry = &dirlist.contents[i];
                    contents
                        .iter()
                        .enumerate()
                        .find(|(_, e)| e.file_name() == entry.file_name())
                        .map(|(i, _)| i)
                        .or(Some(i))
                }
                None => Some(0),
            },
            None => Some(0),
        }
    };

    let metadata = Metadata::from(path)?;
    let dirlist = DirList::new(path.to_path_buf(), contents, index, metadata);

    Ok(dirlist)
}

pub fn read_directory<F>(
    path: &Path,
    filter_func: F,
    options: &DisplayOption,
) -> io::Result<Vec<DirEntry>>
where
    F: Fn(&Result<fs::DirEntry, io::Error>) -> bool,
{
    let results: Vec<DirEntry> = fs::read_dir(path)?
        .filter(filter_func)
        .filter_map(|res| DirEntry::from(&res.ok()?, options).ok())
        .collect();

    Ok(results)
}
