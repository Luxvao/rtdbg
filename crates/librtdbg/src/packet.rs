use std::{io::Read, os::unix::net::UnixStream};

use crate::{
    api::{ReqApi, RespApi},
    error::Error,
};

// Generic struct for a packet
#[derive(Debug, Clone)]
pub struct Packet {
    action: u8,
    payload: Vec<u8>,
}

// Trait implementations
impl From<Packet> for Vec<u8> {
    fn from(mut val: Packet) -> Self {
        let mut packet_vec: Vec<u8> = Vec::new();

        // Push the action
        packet_vec.push(val.action);

        // Push the payload size
        let mut payload_size = val.payload.len().to_le_bytes().to_vec();

        packet_vec.append(&mut payload_size);

        // Push the actual payload
        packet_vec.append(&mut val.payload);

        packet_vec
    }
}

// ReqApi related functions
impl From<ReqApi> for Packet {
    fn from(value: ReqApi) -> Self {
        let action: u8 = value.clone().into();

        let payload = match value {
            ReqApi::Disconnect => Vec::new(),
            ReqApi::AddToQueue { script } => script.get_contents().as_bytes().into(),
            ReqApi::RemoveFromQueue { index } => index.to_le_bytes().into(),
        };

        Packet { action, payload }
    }
}

// RespApi related functions
impl From<RespApi> for Packet {
    fn from(value: RespApi) -> Self {
        let action: u8 = value.clone().into();

        let payload = match value {
            RespApi::Success => Vec::new(),
            RespApi::Error(e) => e.as_bytes().into(),
        };

        Packet { action, payload }
    }
}

// Custom function implementations
impl Packet {
    pub fn read_from_stream(stream: &mut UnixStream) -> Result<Packet, Error> {
        // Pull the header
        let mut header: [u8; 9] = [0; 9];

        stream.read_exact(&mut header)?;

        // Extract the payload size
        let payload_size: usize = usize::from_le_bytes(header[1..9].try_into()?);

        // Extract the payload
        let mut payload: Vec<u8> = vec![0; payload_size];

        stream.read_exact(&mut payload)?;

        Ok(Packet {
            action: header[0],
            payload,
        })
    }

    // Extract the action
    pub fn get_action(&self) -> &u8 {
        &self.action
    }

    // Extract the payload
    pub fn get_payload(&self) -> &Vec<u8> {
        &self.payload
    }
}
