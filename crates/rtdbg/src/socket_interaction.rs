use std::{fs::File, io::Read, path::PathBuf};

use color_eyre::eyre::Result;
use librtdbg::{
    api::{Request, Response},
    runtime_connection::connect,
};
use serde::Deserialize;

pub fn load_script(pid: i32, path: PathBuf) -> Result<()> {
    let mut script_file = File::open(path)?;

    let mut script_contents = String::new();

    script_file.read_to_string(&mut script_contents)?;

    let load_script_req = Request::LoadScript {
        script: script_contents,
    };

    // Create the connection
    let mut socket = connect(pid as u32)?;

    serde_json::to_writer(&mut socket, &load_script_req)?;

    let mut de = serde_json::Deserializer::from_reader(&mut socket);

    let response = Response::deserialize(&mut de)?;

    println!("{:?}", response);

    Ok(())
}
