use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

use librtdbg::{
    error::{Error, unwrap_or_shutdown},
    runtime_extract,
};

pub fn preload(
    ctx: &egui::Context,
    path: String,
    pid: &mut u32,
    receiver: &mut Option<Receiver<String>>,
) -> Result<(), Error> {
    let ctx = ctx.clone();

    let (send, recv) = mpsc::channel::<String>();

    *receiver = Some(recv);

    // Preload setup
    let runtime = runtime_extract::extract()?;

    let mut child = Command::new(path)
        .env("LD_PRELOAD", runtime)
        .stdout(Stdio::piped())
        .spawn()?;

    *pid = child.id();

    thread::spawn(move || {
        let stdout = child.stdout.take();

        if let Some(stdout) = stdout {
            let mut bufreader = BufReader::new(stdout);

            loop {
                let mut buffer = String::new();

                unwrap_or_shutdown(bufreader.read_line(&mut buffer));

                unwrap_or_shutdown(send.send(buffer));

                ctx.request_repaint();
            }
        }
    });

    Ok(())
}
