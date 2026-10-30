use std::io::Write;
use std::path::PathBuf;

use crate::error::Error;
use crate::os::{NativeRuntimeProvider, RuntimeProvider};

pub const RTDBG_RUNTIME: &[u8] = include_bytes!("../../../target/release/librtdbg_runtime.so");

pub fn extract_runtime() -> Result<PathBuf, Error> {
    let mut runtime_file = NativeRuntimeProvider::runtime_file_handle()?;

    runtime_file.write_all(RTDBG_RUNTIME)?;

    Ok(NativeRuntimeProvider::runtime_path()?)
}
