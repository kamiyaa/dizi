use std::io;
use std::path::Path;
use std::process::Command;

pub fn get_mimetype(p: &Path) -> io::Result<String> {
    let output = Command::new("file")
        .arg("-b")
        .arg("--mime-type")
        .arg(p)
        .output()?;
    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    let mimetype = stdout.to_string();

    Ok(mimetype)
}

pub fn is_audio(p: &Path) -> io::Result<bool> {
    let mimetype = get_mimetype(p)?;

    if mimetype.starts_with("audio/") {
        Ok(true)
    } else {
        Ok(false)
    }
}
