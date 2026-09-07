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
    PostcardError(#[from] postcard::Error),
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
    #[error("Failed to parse provided ELF header")]
    ElfHeaderParsingError,
    #[error("Failed to parse provided program header")]
    ProgramHeaderParsingError,
    #[error(
        "Loading a CStr from a register is not possible. If you wish to load a CStr pointer to by a register, use a pointer"
    )]
    CannotLoadCStrFromRegister,
    #[error("Storing a CStr to a register is not possible.")]
    CannotStoreCStrToRegister,
}
