pub mod ipc;
pub mod linux;
pub mod memory;
pub mod runtime;

#[cfg(target_os = "linux")]
pub type NativeRuntimeProvider = linux::runtime_provider::LinuxRuntimeProvider;

#[cfg(target_os = "linux")]
pub type NativeParser = imep::parser::ElfParser;

#[cfg(target_os = "linux")]
pub const NATIVE_PARSER_EXTRA: imep::parser::ParseInfo = imep::parser::ParseInfo::new(
    imep::parser::Libc::COMPILED_WITH,
    imep::Class::NATIVE,
    imep::Endianness::NATIVE,
    imep::Machine::NATIVE,
);
