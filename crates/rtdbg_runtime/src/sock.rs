use std::{
    os::unix::net::{UnixListener, UnixStream},
    process::exit,
    sync::LazyLock,
};

use librtdbg::api::{Request, Response};
use log::{error, info};

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
        let mut buf = [0u8; 256];

        let request: Request = match postcard::from_io((&mut stream, &mut buf)) {
            Ok((r, (_, _))) => r,
            Err(e) => {
                let resp = Response::Error(e.to_string());

                report(&mut stream, resp);

                continue;
            }
        };

        let resp = match request {
            Request::LoadScript { script } => {
                todo!()
            }
            Request::UnloadScript { script_id } => todo!(),
            Request::Status => todo!(),
        };

        report(&mut stream, resp);
    }
}

fn report(stream: &mut UnixStream, resp: Response) {
    if let Err(e) = postcard::to_io(&resp, stream) {
        info!("{e}");
    }
}
