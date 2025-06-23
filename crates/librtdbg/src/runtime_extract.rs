use std::{fs::File, io::Write};

use crate::error::Error;

const RTDBG_RUNTIME_PATH: &str = "/tmp/rtdbg.so";
const DBG_RUNTIME: &[u8] = include_bytes!("../../../target/release/librtdbg_runtime.so");

pub fn extract() -> Result<String, Error> {
    let mut file = File::create(RTDBG_RUNTIME_PATH)?;

    file.write_all(DBG_RUNTIME)?;

    Ok(RTDBG_RUNTIME_PATH.to_string())
}
