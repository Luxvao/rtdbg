use std::sync::PoisonError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    SliceError(#[from] std::array::TryFromSliceError),
    #[error("{0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("{0}")]
    RhaiParseError(#[from] rhai::ParseError),
    #[error("{0}")]
    EvalAltResult(#[from] Box<rhai::EvalAltResult>),
    #[error("Received invalid permissions byte, {0}")]
    PermissionsError(String),
    #[error("Received invalid VMA map, {0}")]
    VmaError(String),
    #[error("Mutex poisoned")]
    MutexPoisoned,
    #[error(
        "Loading a CStr from a register is not possible. If you wish to load a CStr pointer to by a register, use a pointer"
    )]
    CannotLoadCStrFromRegister,
    #[error("Storing a CStr to a register is not possible")]
    CannotStoreCStrToRegister,
    #[error("No such script exists")]
    NoSuchScript,
    #[error("HookSpace is in a bad state")]
    HookSpaceBadState,
    #[error("Architecture associated with {0} is not recognised")]
    UnknownArchitecture(u64),
    #[error("Unable to allocate hook")]
    UnableToAllocateHook,
    #[error("Unable to free hook")]
    UnableToFreeHook,
    #[error("Unable to allocate page")]
    UnableToAllocatePage,
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Self {
        Error::MutexPoisoned
    }
}
