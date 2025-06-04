use std::{fs::File, io::Read, os::unix::net::UnixStream, path::Path, process::exit};

use librtdbg::{api::ReqApi, comms::send_packet, packet::Packet, script::Script};

pub fn inject(pid: String, script: String) {
    let rtdbg_socket = format!("/tmp/rtdbg-{}.sock", pid);

    let rtdbg_socket = Path::new(&rtdbg_socket);

    if !rtdbg_socket.exists() {
        println!("Incorrect PID entered!");
        exit(1);
    }

    let Ok(mut stream) = UnixStream::connect(rtdbg_socket) else {
        println!("Unable to connect to rtdbg socket! Try restarting the debuggee");
        exit(1);
    };

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

    let script = match Script::try_from(file_contents_buffer) {
        Ok(script) => script,
        Err(_) => {
            println!("Script was not UTF8!");
            exit(1);
        }
    };

    let req = ReqApi::AddToQueue { script };

    let packet = Packet::from(req);

    let result = send_packet(&mut stream, packet);

    if result.is_err() {
        println!("Unable to write to rtdbg socket!");
        exit(1);
    }

    let resp = Packet::read_from_stream(&mut stream);

    // We don't really care if it succeeds or not
    let _ = send_packet(&mut stream, Packet::from(ReqApi::Disconnect));

    println!("{:?}", resp);
}
