#[cfg(feature = "include_runtime")]
use std::fs::File;

#[cfg(feature = "include_runtime")]
use std::io::Write;

#[cfg(feature = "include_runtime")]
use crate::error::Error;

pub const RTDBG_RUNTIME_PATH: &str = "/tmp/rtdbg.so";
#[cfg(feature = "include_runtime")]
pub const DBG_RUNTIME: &[u8] = include_bytes!("../../../target/release/librtdbg_runtime.so");

#[cfg(feature = "include_runtime")]
pub fn extract() -> Result<String, Error> {
    let mut file = File::create(RTDBG_RUNTIME_PATH)?;

    file.write_all(DBG_RUNTIME)?;

    Ok(RTDBG_RUNTIME_PATH.to_string())
}

pub fn clean() {
    let _ = std::fs::remove_file(RTDBG_RUNTIME_PATH);
}
