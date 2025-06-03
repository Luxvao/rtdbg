use std::{
    fs::{File, remove_file},
    io::Write,
    path::Path,
    process::{Command, exit},
};

use nix::{sys::ptrace::Options, unistd::Pid};

use crate::{DBG_RUNTIME, RTDBG_RUNTIME_PATH};

pub fn start(preload_f: bool, program: String) {
    match preload_f {
        false => use_ptrace(program),
        true => preload(program),
    }
}

fn use_ptrace(pid: String) {
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

fn preload(program_path: String) {
    let program_to_run = Path::new(program_path.as_str());

    if !program_to_run.is_file() {
        println!("The program doesn't exist or isn't a file!");
        exit(1);
    }

    extract_runtime();

    let program = Command::new(program_to_run)
        .env("LD_PRELOAD", RTDBG_RUNTIME_PATH)
        .spawn();

    if let Ok(mut program) = program {
        let _ = program.wait();
    } else {
        println!("Failed to spawn the provided program!");
        exit(1);
    }

    clean_up_runtime();
}

fn extract_runtime() {
    let runtime = File::create(RTDBG_RUNTIME_PATH);

    if let Ok(mut file) = runtime {
        let result = file.write_all(DBG_RUNTIME);

        if let Err(e) = result {
            println!(
                "Unable to write runtime contents into librtdbg.so! Error: {:?}",
                e
            );

            exit(1);
        }
    } else {
        println!("Unable to extract librtdbg.so");
        exit(1);
    }
}

fn clean_up_runtime() {
    let result = remove_file(RTDBG_RUNTIME_PATH);

    if let Err(e) = result {
        println!("Unable to remove librtdbg.so! Error: {:?}", e);
        exit(1);
    }
}
