use std::{
    collections::VecDeque,
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use librtdbg::{
    error::Error,
    extract::extract_runtime,
    os::{NativeRuntimeProvider, RuntimeProvider},
};

pub fn launch(
    path: &str,
    stdout: Arc<Mutex<VecDeque<String>>>,
) -> Result<(UnixStream, u32), Error> {
    let runtime_path = extract_runtime()?;

    let command_handle = Command::new(&path)
        .env("LD_PRELOAD", runtime_path)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()?;

    let pid = command_handle.id();

    if let Some(c_stdout) = command_handle.stdout {
        std::thread::spawn(move || {
            let mut reader = BufReader::new(c_stdout);

            let mut line = String::new();

            while let Ok(v) = reader.read_line(&mut line)
                && v > 0
            {
                match stdout.lock() {
                    Ok(mut l) => l.push_front(std::mem::take(&mut line)),
                    Err(_) => break,
                }

                line.clear();
            }
        });
    }

    let _ = NativeRuntimeProvider::wait_for_socket(pid);

    Ok((NativeRuntimeProvider::connect_socket(pid)?, pid))
}
