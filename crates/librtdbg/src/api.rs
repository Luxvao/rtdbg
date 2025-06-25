use crate::{error::Error, packet::Packet, script::Script};

/*
    Packet format: <action><payload_size><payload> - General packet
    Also allowed: <action>0 - Non-payload packets
*/

// Possible requests
#[derive(Debug, Clone)]
pub enum ReqApi {
    Disconnect,
    Shutdown,
    AddToQueue { script: Script },
    RemoveFromQueue { index: usize },
}

// Possible responses
#[derive(Debug, Clone)]
pub enum RespApi {
    Success,
    Failure(String),
}

impl TryFrom<Packet> for ReqApi {
    type Error = Error;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        match *packet.get_action() {
            0 => Ok(ReqApi::Disconnect),
            1 => Ok(ReqApi::Shutdown),
            2 => Ok(ReqApi::AddToQueue {
                script: Script::try_from(packet.get_payload().clone())?,
            }),
            3 => Ok(ReqApi::RemoveFromQueue {
                index: usize::from_le_bytes(packet.get_payload().clone().try_into()?),
            }),
            _ => Err(Error::from("Invalid packet received!")),
        }
    }
}

impl TryFrom<Packet> for RespApi {
    type Error = Error;

    fn try_from(packet: Packet) -> Result<Self, Self::Error> {
        match *packet.get_action() {
            0 => Ok(RespApi::Success),
            1 => Ok(RespApi::Failure(String::from_utf8(
                packet.get_payload().clone(),
            )?)),
            _ => Err(Error::from("Invalid packet received!")),
        }
    }
}

// Trait implementations for built-ins
impl From<ReqApi> for u8 {
    fn from(val: ReqApi) -> Self {
        match val {
            ReqApi::Disconnect => 0,
            ReqApi::Shutdown => 1,
            ReqApi::AddToQueue { script: _ } => 2,
            ReqApi::RemoveFromQueue { index: _ } => 3,
        }
    }
}

impl From<RespApi> for u8 {
    fn from(val: RespApi) -> Self {
        match val {
            RespApi::Success => 0,
            RespApi::Failure(_) => 1,
        }
    }
}
