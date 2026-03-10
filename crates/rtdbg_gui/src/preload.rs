use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::{
        Mutex,
        mpsc::{self, Receiver},
    },
    thread,
};

use librtdbg::{error::Error, runtime_extract};

pub static CHILD: Mutex<Option<Child>> = Mutex::new(None);

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

        *CHILD.lock().unwrap() = Some(child);

        if let Some(stdout) = stdout {
            let mut bufreader = BufReader::new(stdout);

            loop {
                let mut buffer = String::new();

                bufreader.read_line(&mut buffer).unwrap();

                send.send(buffer).unwrap();

                ctx.request_repaint();
            }
        }
    });

    Ok(())
}
