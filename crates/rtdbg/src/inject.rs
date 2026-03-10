use std::{fs::File, io::Read, os::unix::net::UnixStream, path::Path, process::exit};

use color_eyre::eyre::{Context, Result};
use librtdbg::{api::ReqApi, comms::send_packet, packet::Packet, script::Script};

pub fn inject(pid: String, script: String) -> Result<()> {
    let rtdbg_socket = format!("/tmp/rtdbg-{}.sock", pid);

    let rtdbg_socket = Path::new(&rtdbg_socket);

    if !rtdbg_socket.exists() {
        println!("Incorrect PID entered!");
        exit(1);
    }

    let mut stream = UnixStream::connect(rtdbg_socket)
        .with_context(|| "Unable to connect to the rtdbg socket")?;

    let mut script_file = File::open(script)?;

    let mut file_contents_buffer: Vec<u8> = Vec::new();

    script_file.read_to_end(&mut file_contents_buffer)?;

    let script = Script::try_from(file_contents_buffer)?;

    let req = ReqApi::AddToQueue { script };

    let packet = Packet::from(req);

    send_packet(&mut stream, packet)?;

    let resp = Packet::read_from_stream(&mut stream);

    // We don't really care if it succeeds or not
    let _ = send_packet(&mut stream, Packet::from(ReqApi::Disconnect));

    println!("{:?}", resp);

    Ok(())
}
