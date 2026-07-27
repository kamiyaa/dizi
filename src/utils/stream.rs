use std::io;
use std::io::Write;
use std::os::unix::net::UnixStream;

pub const NEWLINE: &[u8] = b"\n";

pub fn flush(stream: &mut UnixStream) -> io::Result<()> {
    stream.write_all(NEWLINE)?;
    Ok(())
}
