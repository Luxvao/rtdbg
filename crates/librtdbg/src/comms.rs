use std::os::unix::net::UnixStream;

use serde::Deserialize;

use crate::{
    api::{Request, Response},
    error::Error,
};

pub fn send_request(socket: &UnixStream, request: &Request) -> Result<Response, Error> {
    serde_json::to_writer(socket, request)?;

    let mut de = serde_json::Deserializer::from_reader(socket);

    Ok(Response::deserialize(&mut de)?)
}
