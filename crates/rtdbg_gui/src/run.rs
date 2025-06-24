use std::{
    os::unix::net::UnixStream,
    sync::{Mutex, PoisonError},
};

use librtdbg::{api::ReqApi, comms::send_packet, error::Error, packet::Packet, script::Script};

pub static STREAM: Mutex<Option<UnixStream>> = Mutex::new(None);

pub fn run_script(script_contents: String) -> Result<(), Error> {
    // Just doing PoisonError -> String here. I can't be bothered to deal with From<PoisonError<T>>
    let Some(ref mut socket) = *STREAM
        .lock()
        .map_err(|e| PoisonError::new(Box::new(e.into_inner()) as Box<dyn std::fmt::Debug>))?
    else {
        return Err(Error::OtherError(
            "No binary process being analysed!".to_string(),
        ));
    };

    // Send script
    let script = Script::from(script_contents);

    let add_to_queue_req = ReqApi::AddToQueue { script };

    let add_to_queue_packet = Packet::from(add_to_queue_req);

    send_packet(socket, add_to_queue_packet)?;

    // Send disconnect packet
    let disconnect = ReqApi::Disconnect;

    let disconnect_packet = Packet::from(disconnect);

    send_packet(socket, disconnect_packet)?;

    Ok(())
}
