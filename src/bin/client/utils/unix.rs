use nix::sys::stat::Mode;

const LIBC_EXECUTE_VALS: [Mode; 3] = [Mode::S_IXUSR, Mode::S_IXGRP, Mode::S_IXOTH];

pub fn is_executable(mode: Mode) -> bool {
    LIBC_EXECUTE_VALS.iter().any(|val| mode.intersects(*val))
}
