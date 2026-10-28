use std::{
    io::{BufRead, BufReader},
    os::unix::net::UnixStream,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use librtdbg::{
    error::Error,
    runtime_connection::{connect, wait_for_socket},
    runtime_extract::extract,
};

pub fn launch(path: &str, stdout: Arc<Mutex<String>>) -> Result<(UnixStream, u32), Error> {
    let runtime = extract()?;

    let command_handle = Command::new(&path)
        .env("LD_PRELOAD", runtime)
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
                    Ok(mut l) => l.push_str(&line),
                    Err(_) => break,
                }

                line.clear();
            }
        });
    }

    let _ = wait_for_socket(pid);

    Ok((connect(pid)?, pid))
}
