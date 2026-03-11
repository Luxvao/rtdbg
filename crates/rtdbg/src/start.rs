use std::{
    path::Path,
    process::{Command, exit},
};

use color_eyre::eyre::{Context, Result};
use librtdbg::runtime_extract::{RTDBG_RUNTIME_PATH, clean, extract};
use nix::{sys::ptrace::Options, unistd::Pid};

pub fn start(preload_f: bool, program: String) -> Result<()> {
    if preload_f { use_preload(program) } else { use_ptrace(program) }
}

fn use_ptrace(pid: String) -> Result<()> {
    let Ok(pid) = pid.parse::<i32>() else {
        println!("Invalid PID provided!");
        exit(1);
    };

    let result = nix::sys::ptrace::seize(Pid::from_raw(pid), Options::empty());

    if let Err(e) = result {
        println!("Unable to seize the process. Please try LD_PRELOAD. Error: {e}");
        exit(1);
    }

    panic!("Unsupported for now")
}

fn use_preload(program_path: String) -> Result<()> {
    let program_to_run = Path::new(program_path.as_str());

    if !program_to_run.is_file() {
        eprintln!("The program doesn't exist or isn't a file!");
        exit(1);
    }

    extract()?;

    Command::new(program_to_run)
        .env("LD_PRELOAD", RTDBG_RUNTIME_PATH)
        .status()
        .with_context(|| "Unable to spawn the child process")?;

    clean();

    Ok(())
}
