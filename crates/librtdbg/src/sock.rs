use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    path::Path,
    process::exit,
};

use log::{error, warn};

// Client actions definition
pub type Action = u8;

// Runtime responses definition
pub type Response = [u8; 1];

// Runtime responses
pub const SUCCESS: Response = [0];
pub const FAILED_TO_READ_SOCKET: Response = [1];
pub const MALFORMED_PACKET: Response = [2];
pub const SCRIPT_NOT_UTF8: Response = [3];
pub const UNABLE_TO_ADD_TO_QUEUE: Response = [4];
pub const UNABLE_TO_REMOVE_FROM_QUEUE: Response = [5];
pub const UNABLE_TO_REMOVE_FROM_QUEUE_NO_SUCH_SCRIPT: Response = [6];

// API enum
pub enum Api {
    AddToQueue = 1,
    RemoveFromQueue = 2,
    Disconnect = 3,
}

impl TryFrom<u8> for Api {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Api::AddToQueue),
            2 => Ok(Api::RemoveFromQueue),
            3 => Ok(Api::Disconnect),
            _ => Err(()),
        }
    }
}

pub fn send_script(rtdbg_socket: &Path, mut script: Vec<u8>) -> Response {
    let mut packet: Vec<u8> = Vec::new();

    // Push the action number. Add to queue
    packet.push(Api::AddToQueue as u8);

    let script_size = script.len();

    let mut script_size_as_bytes = script_size.to_le_bytes().to_vec();

    // We append the payload size to the packet
    packet.append(&mut script_size_as_bytes);

    // Lastly we add the script itself
    packet.append(&mut script);

    // Now we can connect to the socket and send the packet
    let Ok(mut client) = UnixStream::connect(rtdbg_socket) else {
        println!("Unable to connect to the socket!");
        exit(1);
    };

    let write_result = client.write_all(&packet);

    if let Err(e) = write_result {
        println!("Unable to write to socket! Error: {:?}", e);
        exit(1);
    }

    let mut response_buffer: [u8; 1] = [0; 1];

    let read_result = client.read_exact(&mut response_buffer);

    if let Err(e) = read_result {
        println!("Unable to read from socket! Error: {:?}", e);
        exit(1);
    }

    response_buffer
}

pub fn remove_script(rtdbg_socket: &Path, script_index: usize) -> Response {
    let mut packet: Vec<u8> = Vec::new();

    // Push the action number for removing from queue
    todo!()
}

pub fn notify_error(msg: &str, stream: &mut UnixStream, error: std::io::Error, response: Response) {
    error!("{}{:?}", msg, error);

    let _ = stream.write_all(&response);
}

pub fn notify_warning(msg: &str, stream: &mut UnixStream, response: Response) {
    warn!("{}", msg);

    let _ = stream.write_all(&response);
}
