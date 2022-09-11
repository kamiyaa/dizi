pub fn is_executable(mode: u32) -> bool {
    const LIBC_PERMISSION_VALS: [u32; 3] = [
        libc::S_IXUSR as u32,
        libc::S_IXGRP as u32,
        libc::S_IXOTH as u32,
    ];

    LIBC_PERMISSION_VALS.iter().any(|val| mode & *val != 0)
}
