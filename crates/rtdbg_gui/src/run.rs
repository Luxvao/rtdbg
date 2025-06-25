use std::{os::unix::net::UnixStream, sync::Mutex};

use librtdbg::{
    api::ReqApi, comms::send_packet, error::Error, packet::Packet, runtime_connection::connect,
    script::Script,
};

pub static STREAM: Mutex<Option<UnixStream>> = Mutex::new(None);

pub fn run_script(pid: u32, script_contents: String) -> Result<(), Error> {
    // Just doing PoisonError -> String here. I can't be bothered to deal with From<PoisonError<T>>
    let mut stream = STREAM.lock()?;

    if stream.is_none() {
        *stream = Some(connect(pid)?);
    }

    // This just can't fail
    let stream = stream.as_mut().unwrap();

    // Send script
    let script = Script::from(script_contents);

    let add_to_queue_req = ReqApi::AddToQueue { script };

    let add_to_queue_packet = Packet::from(add_to_queue_req);

    send_packet(stream, add_to_queue_packet)?;

    Ok(())
}
