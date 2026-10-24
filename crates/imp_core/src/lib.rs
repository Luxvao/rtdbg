use rhai::{CustomType, TypeBuilder};
use thiserror::Error;

pub trait Parser {
    fn import_slots<T>(extra: T) -> impl Iterator<Item = ImportSlot>;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct ImportSlot {
    pub import_name: String,
    pub module_name: String,
    pub ptr: u64,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Parsing ELF header failed")]
    ElfHeaderParsingError,
    #[error("Parsing program header failed")]
    ProgramHeaderParsingError,
}

impl ImportSlot {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .on_print(|v| format!("{v:?}"))
            .on_debug(|v| format!("{v:?}"));
    }
}
