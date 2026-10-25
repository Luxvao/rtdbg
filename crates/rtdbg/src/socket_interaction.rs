use std::{fs::File, io::Read, path::Path};

use color_eyre::eyre::Result;
use librtdbg::{
    api::{Request, Response},
    runtime_connection::{connect, send_request},
    script::ScriptId,
};

pub fn load_script(pid: i32, path: &Path) -> Result<()> {
    let mut script_file = File::open(path)?;

    let mut script_contents = String::new();

    script_file.read_to_string(&mut script_contents)?;

    let load_script_req = Request::LoadScript {
        script: script_contents,
    };

    // Create the connection
    let mut socket = connect(pid as u32)?;

    let response = send_request(&mut socket, &load_script_req)?;

    match response {
        Response::LoadSuccess { script_id } => {
            println!("Script loaded successfully! Script ID: {script_id:?}")
        }
        a => println!("Unexpected response: {a:?}"),
    }

    Ok(())
}

pub fn unload_script(pid: i32, script_id: &ScriptId) -> Result<()> {
    let request = Request::UnloadScript {
        script_id: *script_id,
    };

    let mut socket = connect(pid as u32)?;

    let response = send_request(&mut socket, &request)?;

    match response {
        Response::Success => println!("Unloaded successfully!"),
        a => println!("Unexpected response: {a:?}"),
    }

    Ok(())
}

pub fn script_functions(pid: i32, script_id: &ScriptId) -> Result<()> {
    let request = Request::ScriptFunctions {
        script_id: *script_id,
    };

    let mut socket = connect(pid as u32)?;

    let response = send_request(&mut socket, &request)?;

    match response {
        Response::ScriptFunctions { functions } => {
            for (function, parameters) in functions {
                println!("⚬ `{function}`\n   ⮡ {parameters:?}");
            }
        }
        a => println!("Unexpected response: {a:?}"),
    }

    Ok(())
}

pub fn run_function(
    pid: i32,
    script_id: &ScriptId,
    function: String,
    parameter: Option<&str>,
) -> Result<()> {
    let parameter = match parameter {
        Some(parameter) => Some(serde_json::from_str::<serde_json::Value>(&parameter)?),
        None => None,
    };

    let request = Request::RunFunction {
        script_id: *script_id,
        function,
        parameter,
    };

    let mut socket = connect(pid as u32)?;

    let response = send_request(&mut socket, &request)?;

    match response {
        Response::RunSuccess { response } => println!("Response: {response}"),
        a => println!("Unexpected response: {a:?}"),
    }

    Ok(())
}
