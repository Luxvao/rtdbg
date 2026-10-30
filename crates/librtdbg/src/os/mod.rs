use std::{
    fs::File,
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    time::Duration,
};

use crate::error::Error;

pub mod linux;

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

pub trait RuntimeProvider {
    fn runtime_path() -> Result<PathBuf, Error>;

    fn runtime_file_handle() -> Result<File, Error>;

    fn create_socket(pid: u32) -> Result<UnixListener, Error>;
    fn connect_socket(pid: u32) -> Result<UnixStream, Error>;
    fn wait_for_socket_timeout(pid: u32, timeout: Duration) -> Result<(), Error>;

    /// Waits for the associated socket to appear
    /// *Timeout*: 5s
    fn wait_for_socket(pid: u32) -> Result<(), Error> {
        Self::wait_for_socket_timeout(pid, Duration::from_secs(5))
    }
}
