use std::{fs, io, path, time};

use nix::sys::stat::{Mode, SFlag};

#[cfg(target_os = "macos")]
use nix::sys::stat::mode_t;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AppFileType {
    Directory,
    File,
    // Unix specific
    Link,
    Socket,
    Block,
    Character,
    Pipe,
}

impl From<SFlag> for AppFileType {
    fn from(value: SFlag) -> Self {
        Self::from_mode(value)
    }
}

impl AppFileType {
    pub fn from_mode(mode: SFlag) -> Self {
        match mode {
            SFlag::S_IFBLK => AppFileType::Block,
            SFlag::S_IFCHR => AppFileType::Character,
            SFlag::S_IFDIR => AppFileType::Directory,
            SFlag::S_IFIFO => AppFileType::Pipe,
            SFlag::S_IFLNK => AppFileType::Link,
            SFlag::S_IFSOCK => AppFileType::Socket,
            _ => AppFileType::File,
        }
    }
}

#[derive(Clone, Debug)]
pub enum AppLinkType {
    Normal,
    Symlink { target: String, valid: bool },
}

#[derive(Clone, Debug)]
pub struct AppFileMetadata {
    pub len: u64,
    pub directory_size: Option<usize>,
    pub modified: time::SystemTime,
    pub accessed: time::SystemTime,
    pub mode: Mode,
    pub file_type: AppFileType,
    pub link_type: AppLinkType,
    #[cfg(unix)]
    pub uid: u32,
    #[cfg(unix)]
    pub gid: u32,
}

impl AppFileMetadata {
    pub fn from(path: &path::Path) -> io::Result<Self> {
        #[cfg(unix)]
        use std::os::unix::fs::MetadataExt;

        let symlink_metadata = fs::symlink_metadata(path)?;
        let is_symlink = symlink_metadata.file_type().is_symlink();
        // only symlinks need a second stat to resolve their target
        let metadata = if is_symlink {
            fs::metadata(path)
        } else {
            Ok(symlink_metadata.clone())
        };
        let (len, modified, accessed) = match metadata.as_ref() {
            Ok(m) => (m.len(), m.modified()?, m.accessed()?),
            Err(_) => (
                symlink_metadata.len(),
                symlink_metadata.modified()?,
                symlink_metadata.accessed()?,
            ),
        };

        let directory_size = None;
        let (file_type, mode) = match metadata.as_ref() {
            Ok(metadata) => {
                let metadata_mode = metadata.mode();
                #[cfg(target_os = "macos")]
                let sflag = SFlag::from_bits_truncate(metadata_mode as mode_t);

                #[cfg(not(target_os = "macos"))]
                let sflag = SFlag::from_bits_truncate(metadata_mode);

                #[cfg(target_os = "macos")]
                let mode = Mode::from_bits_truncate(metadata_mode as mode_t);

                #[cfg(not(target_os = "macos"))]
                let mode = Mode::from_bits_truncate(metadata_mode);

                (AppFileType::from_mode(sflag), mode)
            }
            _ => (AppFileType::File, Mode::empty()),
        };

        let link_type = if is_symlink {
            let mut link = "".to_string();

            if let Ok(path) = fs::read_link(path)
                && let Some(s) = path.to_str()
            {
                link = s.to_string();
            }

            AppLinkType::Symlink {
                target: link,
                valid: metadata.is_ok(),
            }
        } else {
            AppLinkType::Normal
        };

        #[cfg(unix)]
        let uid = symlink_metadata.uid();
        #[cfg(unix)]
        let gid = symlink_metadata.gid();

        Ok(Self {
            len,
            directory_size,
            modified,
            accessed,
            mode,
            file_type,
            link_type,
            #[cfg(unix)]
            uid,
            #[cfg(unix)]
            gid,
        })
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn directory_size(&self) -> Option<usize> {
        self.directory_size
    }

    pub fn update_directory_size(&mut self, size: usize) {
        self.directory_size = Some(size);
    }

    pub fn modified(&self) -> time::SystemTime {
        self.modified
    }

    pub fn accessed(&self) -> time::SystemTime {
        self.accessed
    }

    pub fn file_type(&self) -> AppFileType {
        self.file_type
    }

    pub fn link_type(&self) -> &AppLinkType {
        &self.link_type
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == AppFileType::Directory
    }
}
