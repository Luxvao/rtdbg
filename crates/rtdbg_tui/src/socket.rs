use std::os::unix::net::UnixStream;

use librtdbg::{
    api::{Request, Response},
    comms::send_request,
    error::Error,
    script::ScriptId,
};

pub fn remote_scripts(socket: &UnixStream) -> Result<Vec<(ScriptId, String)>, Error> {
    let request = Request::Status;

    let resp = send_request(socket, &request)?;

    if let Response::Status { loaded_scripts } = resp {
        Ok(loaded_scripts)
    } else {
        Err(Error::UnexpectedResponse(resp))
    }
}

pub fn load_script(socket: &UnixStream, src: String) -> Result<(), Error> {
    let request = Request::LoadScript { script: src };

    let resp = send_request(socket, &request)?;

    if let Response::LoadSuccess { script_id: _ } = resp {
        Ok(())
    } else {
        Err(Error::UnexpectedResponse(resp))
    }
}

pub fn unload_script(socket: &UnixStream, id: ScriptId) -> Result<(), Error> {
    let request = Request::UnloadScript { script_id: id };

    let resp = send_request(socket, &request)?;

    if let Response::Success = resp {
        Ok(())
    } else {
        Err(Error::UnexpectedResponse(resp))
    }
}
