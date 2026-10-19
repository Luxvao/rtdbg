use thiserror::Error;

pub trait Parser {
    fn import_slots<T>(extra: T) -> impl Iterator<Item = ImportSlot>;
}

#[derive(Clone, Debug)]
pub struct ImportSlot {
    pub function: String,
    pub module: String,
    pub ptr: u64,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parsing ELF header failed")]
    ElfHeaderParsingError,
    #[error("Parsing program header failed")]
    ProgramHeaderParsingError,
}
