use std::{fs::File, io::Read, path::Path, process::exit};

use librtdbg::sock::send_script;

pub fn inject(pid: String, script: String) {
    let rtdbg_socket = format!("/tmp/rtdbg-{}.sock", pid);

    let rtdbg_socket = Path::new(&rtdbg_socket);

    if !rtdbg_socket.exists() {
        println!("Incorrect PID entered!");
        exit(1);
    }

    let Ok(mut script_file) = File::open(script) else {
        println!("The script file does not exist!");
        exit(1);
    };

    let mut file_contents_buffer: Vec<u8> = Vec::new();

    let read_result = script_file.read_to_end(&mut file_contents_buffer);

    if let Err(e) = read_result {
        println!("Unable to read from file! Error: {e}");
        exit(1);
    }

    let response = send_script(rtdbg_socket, file_contents_buffer);

    exit(response[0] as i32);
}
