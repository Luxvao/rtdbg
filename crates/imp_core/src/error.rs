use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parsing ELF header failed")]
    ElfHeaderParsingError,
    #[error("Parsing program header failed")]
    ProgramHeaderParsingError,
}
