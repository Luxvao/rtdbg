use std::{
    os::unix::net::{UnixListener, UnixStream},
    process::exit,
    sync::LazyLock,
};

use librtdbg::api::{Request, Response};
use log::{error, info};
use rhai::Dynamic;

use crate::runtime::{load_script, run_function, status, unload_script};

pub static SOCKET_PATH: LazyLock<String> = LazyLock::new(setup_sock);

fn setup_sock() -> String {
    let pid = std::process::id();

    format!("/tmp/rtdbg-{pid}.sock")
}

fn clean_sock() {
    let sock_path = setup_sock();

    let _ = std::fs::remove_file(sock_path);
}

pub fn io() {
    let Ok(listener) = UnixListener::bind(SOCKET_PATH.clone()) else {
        error!("Failed to bind the socket, exiting!");

        exit(1);
    };

    for stream in listener.incoming().flatten() {
        std::thread::spawn(|| handle_client(stream));
    }
}

fn handle_client(mut stream: UnixStream) {
    loop {
        let request: Request = match serde_json::from_reader(&stream) {
            Ok(r) => r,
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
                params,
            } => {
                if let Some(params) = params {
                    run_function::<Dynamic, (Dynamic,)>(
                        script_id,
                        function,
                        (serde_json::from_value(params).unwrap_or_default(),),
                    )
                    .map_or_else(
                        |e| Response::Error(e.to_string()),
                        |o| Response::RunSuccess {
                            response: serde_json::to_value(o).unwrap_or_default(),
                        },
                    )
                } else {
                    run_function::<Dynamic, ()>(script_id, function, ()).map_or_else(
                        |e| Response::Error(e.to_string()),
                        |o| Response::RunSuccess {
                            response: serde_json::to_value(o).unwrap_or_default(),
                        },
                    )
                }
            }
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
