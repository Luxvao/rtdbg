// This example generates the JSON API info for Rhai LSP

use rhai::{Engine, packages::Package};
use rtdbg_runtime::rhai_lib::RTDBG_PACKAGE;

fn main() {
    let mut engine = Engine::new_raw();

    RTDBG_PACKAGE.register_into_engine(&mut engine);

    engine
        .definitions()
        .write_to_file("rtdbglib.d.rhai")
        .unwrap();
}
