use std::os::unix::net::UnixStream;

use crate::error::Error;

pub fn connect(pid: u32) -> Result<UnixStream, Error> {
    let stream = UnixStream::connect(format!("/tmp/rtdbg-{}.sock", pid))?;

    Ok(stream)
}
