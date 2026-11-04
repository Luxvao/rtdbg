use std::{os::unix::net::UnixStream, path::Path};

use librtdbg::{
    api::{Request, Response},
    comms::send_request,
    error::Error,
    os::{NativeRuntimeProvider, RuntimeProvider},
    script::ScriptId,
};
use serde_json::Value;

pub fn interact_remote<T, F>(pid: u32, f: F) -> Result<T, Error>
where
    F: FnOnce(&UnixStream) -> Result<T, Error>,
{
    let stream = NativeRuntimeProvider::connect_socket(pid)?;

    f(&stream)
}

pub fn remote_scripts(socket: &UnixStream) -> Result<Vec<(ScriptId, String)>, Error> {
    let request = Request::Status;

    let resp = send_request(socket, &request)?;

    if let Response::Status { loaded_scripts } = resp {
        Ok(loaded_scripts)
    } else {
        Err(Error::UnexpectedResponse(resp))
    }
}

pub fn load_script(socket: &UnixStream, src: String) -> Result<ScriptId, Error> {
    let request = Request::LoadScript { script: src };

    let resp = send_request(socket, &request)?;

    if let Response::LoadSuccess { script_id } = resp {
        Ok(script_id)
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

pub fn get_rpcs(socket: &UnixStream) -> Result<Vec<(ScriptId, Vec<(String, Vec<String>)>)>, Error> {
    let request = Request::AllRpcs;

    let resp = send_request(socket, &request)?;

    if let Response::AllRpcs { rpcs } = resp {
        Ok(rpcs)
    } else {
        Err(Error::UnexpectedResponse(resp))
    }
}

pub fn run_rpc(
    socket: &UnixStream,
    script_id: &ScriptId,
    rpc: String,
    args: Value,
) -> Result<String, Error> {
    let request = Request::RunFunction {
        script_id: *script_id,
        function: rpc,
        arguments: Some(args),
    };

    let resp = send_request(socket, &request)?;

    if let Response::RunSuccess { response } = resp {
        Ok(response.to_string())
    } else {
        Err(Error::UnexpectedResponse(resp))
    }
}
