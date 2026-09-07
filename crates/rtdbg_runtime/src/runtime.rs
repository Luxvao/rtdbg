use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex},
};

use librtdbg::{
    error::Error,
    script::{Script, ScriptContext, ScriptId, Store},
};
use log::info;
use rhai::{Engine, FuncArgs, Scope, packages::Package};

use crate::rhai_lib::RTDBG_PACKAGE;

pub static GLOBAL_STORE: LazyLock<Store> = LazyLock::new(|| Store::new());

pub static SCRIPT_STORAGE: Mutex<LazyLock<HashMap<ScriptId, ScriptContext>>> =
    Mutex::new(LazyLock::new(|| HashMap::new()));

pub fn runtime() {
    // Set up the logger
    colog::init();

    // Display the PID of this process
    info!("PID: {}", std::process::id());

    loop {}
}

pub fn load_script(script: String) -> Result<ScriptId, Error> {
    let mut engine = Engine::new_raw();
    let mut scope = Scope::new();

    RTDBG_PACKAGE.register_into_engine(&mut engine);

    let script = Script {
        ast: engine.compile(script)?,
    };

    let script_store = Store::new();

    engine.call_fn::<()>(
        &mut scope,
        &script.ast,
        "init",
        (script_store.clone(), GLOBAL_STORE.clone()),
    )?;

    let script_context = ScriptContext {
        script: Arc::new(script),
        store: script_store,
    };

    todo!()
}

pub fn execute_function<T: Clone + Send + Sync + 'static, P: FuncArgs>(
    script: &Script,
    function: String,
    args: P,
) -> Result<T, Error> {
    let mut engine = Engine::new_raw();
    let mut scope = Scope::new();

    RTDBG_PACKAGE.register_into_engine(&mut engine);

    engine
        .call_fn::<T>(&mut scope, &script.ast, function, args)
        .map_err(|e| e.into())
}
