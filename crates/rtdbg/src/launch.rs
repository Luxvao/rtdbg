use std::{
    path::Path,
    process::{Command, Stdio},
};

use color_eyre::Result;
use librtdbg::extract::extract_runtime;

use crate::repl::{ATTACHED_PID, Error};

pub fn launch(program: &Path) -> Result<(), Error> {
    let runtime_path = extract_runtime()?;

    let child = Command::new(program)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .env("LD_PRELOAD", runtime_path)
        .spawn()?;

    let pid = child.id();

    *ATTACHED_PID.write().expect("ATTACHED_PID poisoned") = Some(pid);

    println!("{}", pid);

    Ok(())
}
