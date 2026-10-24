use std::{path::PathBuf, process::Command};

use color_eyre::Result;
use librtdbg::runtime_extract::extract;

pub fn launch(program: PathBuf) -> Result<()> {
    let runtime_path = extract()?;

    let status = Command::new(program)
        .env("LD_PRELOAD", runtime_path)
        .status()?;

    println!("{:?}", status);

    Ok(())
}
