use std::{path::PathBuf, process::Command};

use color_eyre::Result;
use librtdbg::extract::extract_runtime;

pub fn launch(program: PathBuf) -> Result<()> {
    let runtime_path = extract_runtime()?;

    let mut child = Command::new(program)
        .env("LD_PRELOAD", runtime_path)
        .spawn()?;

    println!("PID: {}", child.id());

    let status = child.wait()?;

    println!("{:?}", status);

    Ok(())
}
