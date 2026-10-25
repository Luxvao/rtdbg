use std::os::unix::net::UnixStream;

use serde::Deserialize;

use crate::{
    api::{Request, Response},
    error::Error,
};

pub fn connect(pid: u32) -> Result<UnixStream, Error> {
    let stream = UnixStream::connect(format!("/tmp/rtdbg-{pid}.sock"))?;

    Ok(stream)
}

pub fn send_request(socket: &mut UnixStream, request: &Request) -> Result<Response, Error> {
    serde_json::to_writer(&mut *socket, request)?;

    let mut de = serde_json::Deserializer::from_reader(socket);

    Ok(Response::deserialize(&mut de)?)
}
