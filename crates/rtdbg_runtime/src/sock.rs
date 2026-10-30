use std::os::unix::net::UnixStream;

use librtdbg::{
    api::{Request, Response},
    os::{NativeRuntimeProvider, RuntimeProvider},
};
use log::{error, info};
use rhai::Dynamic;
use serde::Deserialize;

use crate::runtime::{get_script_functions, load_script, run_function, status, unload_script};

pub fn io() {
    let pid = std::process::id();

    let listener = match NativeRuntimeProvider::create_socket(pid) {
        Ok(listener) => listener,
        Err(e) => {
            error!("{e}");
            return;
        }
    };

    for stream in listener.incoming().flatten() {
        std::thread::spawn(|| handle_client(stream));
    }
}

fn handle_client(mut stream: UnixStream) {
    loop {
        let mut de = serde_json::Deserializer::from_reader(&mut stream);

        let request: Request = match Request::deserialize(&mut de) {
            Ok(r) => r,
            Err(e) if e.is_eof() => {
                break;
            }
            Err(e) => {
                let resp = Response::Error(e.to_string());

                report(&mut stream, resp);

                continue;
            }
        };

        let resp = match request {
            Request::LoadScript { script } => load_script(script).map_or_else(
                |e| Response::Error(e.to_string()),
                |v| Response::LoadSuccess { script_id: v },
            ),
            Request::UnloadScript { script_id } => unload_script(script_id)
                .map_or_else(|e| Response::Error(e.to_string()), |_| Response::Success),
            Request::RunFunction {
                script_id,
                function,
                parameter: params,
            } => {
                if let Some(params) = params {
                    run_function::<Dynamic, (Dynamic,)>(
                        script_id,
                        &function,
                        (serde_json::from_value(params).unwrap_or_default(),),
                    )
                    .map_or_else(
                        |e| Response::Error(e.to_string()),
                        |o| Response::RunSuccess {
                            response: serde_json::to_value(o).unwrap_or_default(),
                        },
                    )
                } else {
                    run_function::<Dynamic, ()>(script_id, &function, ()).map_or_else(
                        |e| Response::Error(e.to_string()),
                        |o| Response::RunSuccess {
                            response: serde_json::to_value(o).unwrap_or_default(),
                        },
                    )
                }
            }
            Request::ScriptFunctions { script_id } => get_script_functions(&script_id).map_or_else(
                |e| Response::Error(e.to_string()),
                |o| Response::ScriptFunctions { functions: o },
            ),
            Request::Status => status().map_or_else(
                |e| Response::Error(e.to_string()),
                |scripts| Response::Status {
                    loaded_scripts: scripts,
                },
            ),
        };

        report(&mut stream, resp);
    }
}

fn report(stream: &mut UnixStream, resp: Response) {
    if let Err(e) = serde_json::to_writer(stream, &resp) {
        info!("{e}");
    }
}
