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
    fn rtdbg_path() -> Result<PathBuf, Error>;

    fn runtime_file_handle() -> Result<File, Error>;

    fn socket_path(pid: u32) -> Result<PathBuf, Error>;

    fn create_socket(pid: u32) -> Result<UnixListener, Error>;
    fn connect_socket(pid: u32) -> Result<UnixStream, Error>;
    fn wait_for_socket_timeout(pid: u32, timeout: Duration) -> Result<(), Error>;

    // True means it's available
    fn prune_inactive_socket(pid: u32) -> bool {
        if Self::connect_socket(pid).is_err() {
            if let Ok(sock_path) = Self::socket_path(pid) {
                if sock_path.is_file() {
                    let _ = std::fs::remove_file(sock_path);
                }
            }

            return false;
        }

        true
    }

    /// Waits for the associated socket to appear
    /// *Timeout*: 5s
    fn wait_for_socket(pid: u32) -> Result<(), Error> {
        Self::wait_for_socket_timeout(pid, Duration::from_secs(5))
    }
}
