use crate::{error::Error, packet::Packet, script::Script};

/*
    Packet format: <action><payload_size><payload> - General packet
    Also allowed: <action>0 - Zero-payload packets
*/

// Possible requests
#[derive(Debug, Clone)]
pub enum ReqApi {
    Disconnect,
    AddToQueue { script: Script },
    RemoveFromQueue { index: usize },
}

// Possible responses
#[derive(Debug, Clone)]
pub enum RespApi {
    Success,
    Error(String),
}

impl TryFrom<Packet> for ReqApi {
    type Error = Error;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        match packet.get_action() {
            &0 => Ok(ReqApi::Disconnect),
            &1 => Ok(ReqApi::AddToQueue {
                script: Script::try_from(packet.get_payload().clone())?,
            }),
            &2 => Ok(ReqApi::RemoveFromQueue {
                index: usize::from_le_bytes(packet.get_payload().clone().try_into()?),
            }),
            _ => {
                todo!()
            }
        }
    }
}

// Trait implementations
impl Into<u8> for ReqApi {
    fn into(self) -> u8 {
        match self {
            ReqApi::Disconnect => 0,
            ReqApi::AddToQueue { script: _ } => 1,
            ReqApi::RemoveFromQueue { index: _ } => 2,
        }
    }
}

impl Into<u8> for RespApi {
    fn into(self) -> u8 {
        match self {
            RespApi::Success => 0,
            RespApi::Error(_) => 1,
        }
    }
}
