// This example generates the JSON API info for Rhai LSP

use rhai::{Engine, Scope};
use rtdbg_runtime::rhai_lib;

fn main() {
    let mut engine = Engine::new();

    rhai_lib::setup_functions(&mut engine);
    rhai_lib::setup_types(&mut engine);

    let mut scope = Scope::new();

    rhai_lib::setup_constants(&mut scope);

    let metadata = engine
        .gen_fn_metadata_to_json(false)
        .expect("Failed to generate metadata");

    std::fs::write("./rtdbglib.json", &metadata).expect("Failed to write metadata");
}
