use std::{io::Write, os::unix::net::UnixStream};

use log::info;

use crate::{api::RespApi, error::Error, packet::Packet};

// Reports error over UnixStream
pub fn report_error(stream: &mut UnixStream, e: Error) {
    // Render the error
    let error_str = format!("{e}");

    info!("{error_str}");

    // Turn it into a response
    let resp = RespApi::Error(error_str);

    // Then into a packet
    let packet = Packet::from(resp);

    let write_err = send_packet(stream, packet);

    if let Err(e) = write_err {
        report_write_error(e);
    }
}

pub fn report_write_error(error: Error) {
    // We only report it to the log. No reason to attempt recovery/resync or writing again
    info!("Stream write error: {}", error);
}

// Generic function to send packets
pub fn send_packet(stream: &mut UnixStream, packet: Packet) -> Result<(), Error> {
    let vec: Vec<u8> = packet.into();

    stream.write_all(&vec)?;

    Ok(())
}
