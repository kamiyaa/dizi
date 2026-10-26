use std::cmp;

use crate::config::option::{SortMethod, SortMethodList};
use crate::fs::AppDirEntry;

#[derive(Clone, Debug)]
pub struct FileSortConfig {
    pub directories_first: bool,
    pub case_sensitive: bool,
    pub reverse: bool,
    pub sort_methods: SortMethodList,
}

impl FileSortConfig {
    pub fn set_sort_method(&mut self, method: SortMethod) {
        self.sort_methods.reorganize(method);
    }

    pub fn compare(&self, f1: &AppDirEntry, f2: &AppDirEntry) -> cmp::Ordering {
        if self.directories_first {
            let f1_isdir = f1.metadata.is_dir();
            let f2_isdir = f2.metadata.is_dir();

            if f1_isdir && !f2_isdir {
                return cmp::Ordering::Less;
            } else if !f1_isdir && f2_isdir {
                return cmp::Ordering::Greater;
            }
        }

        // let mut res = self.sort_method.cmp(f1, f2, &self);
        let mut res = self.sort_methods.cmp(f1, f2, self);
        if self.reverse {
            res = match res {
                cmp::Ordering::Less => cmp::Ordering::Greater,
                cmp::Ordering::Greater => cmp::Ordering::Less,
                s => s,
            };
        };
        res
    }
}

impl std::default::Default for FileSortConfig {
    fn default() -> Self {
        FileSortConfig {
            directories_first: true,
            case_sensitive: false,
            reverse: false,
            sort_methods: SortMethodList::default(),
        }
    }
}
