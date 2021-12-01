use std::io;
use std::path::Path;
use std::process::Command;

use log::{debug, log_enabled, Level};

pub fn get_mimetype(p: &Path) -> io::Result<String> {
    let output = Command::new("file")
        .arg("-b")
        .arg("--mime-type")
        .arg(p)
        .output()?;
    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    let mimetype = stdout.to_string();

    if log_enabled!(Level::Debug) {
        debug!("{:?} mimetype: {}", p, mimetype);
    }

    Ok(mimetype)
}

pub fn is_playable(p: &Path) -> io::Result<bool> {
    let mimetype = get_mimetype(p)?;

    Ok(is_mimetype_audio(&mimetype) || is_mimetype_video(&mimetype))
}

pub fn is_mimetype_audio(s: &str) -> bool {
    s.starts_with("audio/")
}

pub fn is_mimetype_video(s: &str) -> bool {
    s.starts_with("video/")
}
