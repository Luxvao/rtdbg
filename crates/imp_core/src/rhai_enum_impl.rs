use rhai::export_module;

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod objectformat_module {

    use crate::ObjectFormat;

    pub const Elf: ObjectFormat = ObjectFormat::Elf;
    pub const Pe: ObjectFormat = ObjectFormat::Pe;
    pub const Macho: ObjectFormat = ObjectFormat::Macho;
}
