use std::io;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub const NEWLINE: &[u8] = &['\n' as u8];

pub fn flush(stream: &mut UnixStream) -> io::Result<()> {
    stream.write(NEWLINE)?;
    Ok(())
}
