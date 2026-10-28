use std::{
    os::unix::net::UnixStream,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use serde::Deserialize;

use crate::{
    api::{Request, Response},
    error::Error,
};

/// Waits for the socket to come up
/// *Timeout*: 5s
pub fn wait_for_socket(pid: u32) -> Option<()> {
    let path = PathBuf::from(format!("/tmp/rtdbg-{pid}.sock"));

    let started = Instant::now();

    while !path.exists() {
        if started.duration_since(Instant::now()) > Duration::from_secs(5) {
            return None;
        }

        thread::sleep(Duration::from_millis(10));
    }

    Some(())
}

pub fn connect(pid: u32) -> Result<UnixStream, Error> {
    let stream = UnixStream::connect(format!("/tmp/rtdbg-{pid}.sock"))?;

    Ok(stream)
}

pub fn send_request(socket: &UnixStream, request: &Request) -> Result<Response, Error> {
    serde_json::to_writer(&*socket, request)?;

    let mut de = serde_json::Deserializer::from_reader(socket);

    Ok(Response::deserialize(&mut de)?)
}
