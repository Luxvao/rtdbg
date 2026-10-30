use std::{
    env,
    fs::{File, OpenOptions},
    os::unix::net::{UnixListener, UnixStream},
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{error::Error, os::RuntimeProvider};

pub struct LinuxRuntimeProvider {}

impl RuntimeProvider for LinuxRuntimeProvider {
    fn runtime_path() -> Result<PathBuf, Error> {
        Ok(Self::rtdbg_temp_dir()?.join("rtdbg_runtime.so"))
    }

    fn runtime_file_handle() -> Result<File, Error> {
        Self::init()?;

        Ok(OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .create(true)
            .open(Self::runtime_path()?)?)
    }

    fn create_socket(pid: u32) -> Result<UnixListener, Error> {
        Self::init()?;

        Ok(UnixListener::bind(Self::socket_path(pid)?)?)
    }

    fn connect_socket(pid: u32) -> Result<UnixStream, Error> {
        Ok(UnixStream::connect(Self::socket_path(pid)?)?)
    }

    fn wait_for_socket_timeout(pid: u32, timeout: Duration) -> Result<(), Error> {
        let timestamp = Instant::now();

        let socket_path = Self::socket_path(pid)?;

        while !socket_path.exists() {
            if Instant::now().duration_since(timestamp) >= timeout {
                return Err(Error::Timeout);
            }

            std::thread::sleep(Duration::from_millis(10));
        }

        Ok(())
    }
}

impl LinuxRuntimeProvider {
    fn init() -> Result<(), Error> {
        std::fs::create_dir_all(Self::rtdbg_temp_dir()?)?;

        Ok(())
    }

    fn user_temp_dir() -> Result<PathBuf, Error> {
        Ok(PathBuf::from(env::var("XDG_RUNTIME_DIR")?))
    }

    fn rtdbg_temp_dir() -> Result<PathBuf, Error> {
        Ok(Self::user_temp_dir()?.join("rtdbg/"))
    }

    fn socket_path(pid: u32) -> Result<PathBuf, Error> {
        Ok(Self::rtdbg_temp_dir()?.join(format!("{}.sock", pid)))
    }
}
