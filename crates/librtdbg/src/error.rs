use std::{array::TryFromSliceError, num::ParseIntError, string::FromUtf8Error};

use thiserror::Error;

use crate::packet::Packet;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("{0}")]
    ParseError(#[from] ParseIntError),
    #[error("{0}")]
    SliceError(#[from] TryFromSliceError),
    #[error("{0}")]
    Utf8Error(#[from] FromUtf8Error),
    #[error("Received an invalid packet: {0:?}")]
    InvalidPacket(Packet),
    #[error("Received invalid permissions byte, {0}")]
    PermissionsError(String),
    #[error("Received invalid VMA map, {0}")]
    VmaError(String),
    #[error("Mutex poisoned")]
    MutexPoisoned,
}
