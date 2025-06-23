use std::process::exit;

use log::{error, info};
use rhai::{Engine, Scope};

use crate::{
    SCRIPT_QUEUE,
    rhai_lib::{self, setup_constants},
};

pub fn runtime() {
    // Set up the logger
    colog::init();

    // Display the PID of this process
    info!("PID: {}", std::process::id());

    // Set up the Rhai engine
    let mut engine = Engine::new();

    rhai_lib::setup_functions(&mut engine);

    let (queue, condvar) = &SCRIPT_QUEUE;

    let Ok(mut queue) = queue.lock() else {
        error!("Unable to lock the queue! Exiting!");

        exit(1);
    };

    let mut scope = Scope::new();

    setup_constants(&mut scope);

    loop {
        queue = condvar.wait(queue).expect("I don't think this will ever happen, but just in case - runtime.rs line 29 - mutex is poisoned");

        let script = queue.pop_front();

        if let Some(script) = script {
            let engine_result = engine.run_with_scope(&mut scope, script.get_contents());

            if let Err(e) = engine_result {
                error!("Unable to execute script! Error: {:?}", e);
            }
        }
    }
}
