use std::io;
use std::path::Path;
use std::process::Command;

pub fn get_mimetype(p: &Path) -> io::Result<String> {
    let output = Command::new("file")
        .arg("-b")
        .arg("--mime-type")
        .arg(p)
        .output()?;
    let stdout = std::str::from_utf8(&output.stdout).expect("Failed to read from stdout");
    let mimetype = stdout.to_string();
    tracing::debug!("{:?} mimetype: {}", p, mimetype);
    Ok(mimetype)
}

pub fn is_playable(p: &Path) -> io::Result<bool> {
    let mimetype = get_mimetype(p)?;
    let is_audio_mimetype = is_mimetype_audio(&mimetype) || is_mimetype_video(&mimetype);
    if is_audio_mimetype {
        return Ok(true);
    }
    match p.extension() {
        None => Ok(false),
        Some(s) => match s.to_string_lossy().as_ref() {
            "aac" | "flac" | "mp3" | "mp4" | "m4a" | "ogg" | "opus" | "wav" | "webm" => Ok(true),
            _ => Ok(false),
        },
    }
}

pub fn is_mimetype_audio(s: &str) -> bool {
    s.starts_with("audio/")
}

pub fn is_mimetype_video(s: &str) -> bool {
    s.starts_with("video/")
}
