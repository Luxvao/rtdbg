use std::fmt::Display;

// Generic error for this library
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    TryFromSliceError(std::array::TryFromSliceError),
    FromUtf8Error(std::string::FromUtf8Error),
    VecU8Error(Vec<u8>),
}

// Trait implementations for Error
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IoError(e) => write!(f, "IO error: {:?}", e),
            Error::TryFromSliceError(e) => write!(f, "Try from slice error: {:?}", e),
            Error::FromUtf8Error(e) => write!(f, "From UTF8 error: {:?}", e),
            Error::VecU8Error(e) => write!(f, "Vec<u8> error: {:?}", e),
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
