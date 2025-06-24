use std::{fmt::Display, process::exit, sync::PoisonError};

use log::warn;

// Generic error for this library
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    TryFromSliceError(std::array::TryFromSliceError),
    FromUtf8Error(std::string::FromUtf8Error),
    VecU8Error(Vec<u8>),
    SendError(std::sync::mpsc::SendError<String>),
    ParseIntError(std::num::ParseIntError),
    PoisonError(std::sync::PoisonError<Box<dyn std::fmt::Debug>>),
    OtherError(String),
}

// Trait implementations for Error
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO error: {:?}", e),
            Error::TryFromSliceError(e) => write!(f, "Try from slice error: {:?}", e),
            Error::FromUtf8Error(e) => write!(f, "From UTF8 error: {:?}", e),
            Error::VecU8Error(e) => write!(f, "Vec<u8> error: {:?}", e),
            Error::SendError(e) => write!(f, "Send error: {:?}", e),
            Error::ParseIntError(e) => write!(f, "Parse int error: {:?}", e),
            Error::PoisonError(e) => write!(f, "Poison error: {:?}", e),
            Error::OtherError(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(value: std::array::TryFromSliceError) -> Self {
        Error::TryFromSliceError(value)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(value)
    }
}

impl From<Vec<u8>> for Error {
    fn from(value: Vec<u8>) -> Self {
        Error::VecU8Error(value)
    }
}

impl From<std::sync::mpsc::SendError<String>> for Error {
    fn from(value: std::sync::mpsc::SendError<String>) -> Self {
        Error::SendError(value)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(value: std::num::ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

impl From<std::sync::PoisonError<Box<dyn std::fmt::Debug>>> for Error {
    fn from(value: std::sync::PoisonError<Box<dyn std::fmt::Debug>>) -> Self {
        Error::PoisonError(value)
    }
}

impl From<&'static str> for Error {
    fn from(value: &'static str) -> Self {
        Error::OtherError(value.to_string())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::OtherError(value)
    }
}

pub fn unwrap_or_shutdown<T, E>(res: Result<T, E>) -> T
where
    Error: From<E>,
{
    match res {
        Ok(t) => t,
        Err(e) => {
            warn!("Error occured, shutting down! Error: {}", Error::from(e));
            exit(1);
        }
    }
}
