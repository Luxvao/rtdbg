use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex, atomic::Ordering},
};

use librtdbg::{
    ID_GEN,
    either::Either,
    error::Error,
    hook::HookId,
    parameter::Parameter,
    script::{Script, ScriptCallback, ScriptContext, ScriptId, Store},
};
use log::{info, warn};
use rhai::{Array, Dynamic, Engine, FnPtr, FuncArgs, Map, Scope, packages::Package};

use crate::rhai_lib::RTDBG_PACKAGE;

pub static GLOBAL_STORE: LazyLock<Store> = LazyLock::new(Store::new);

pub static SCRIPT_STORAGE: LazyLock<Mutex<HashMap<ScriptId, ScriptContext>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static RHAI_HOOK_CALLBACKS: LazyLock<Mutex<HashMap<HookId, ScriptCallback>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn runtime() {
    // Set up the logger
    colog::init();

    // Display the PID of this process
    info!("PID: {}", std::process::id());
}

pub fn load_script(script: String) -> Result<ScriptId, Error> {
    let mut engine = Engine::new_raw();
    let mut scope = Scope::new();

    RTDBG_PACKAGE.register_into_engine(&mut engine);

    let script = Script {
        ast: engine.compile(script)?,
    };

    let script_store = Store::new();

    let script_id = ScriptId(ID_GEN.fetch_add(1, Ordering::Relaxed));

    engine.call_fn::<()>(
        &mut scope,
        &script.ast,
        "init",
        (script_id, script_store.clone(), GLOBAL_STORE.clone()),
    )?;

    let script_context = ScriptContext {
        script: Arc::new(script),
        store: script_store,
    };

    SCRIPT_STORAGE.lock()?.insert(script_id, script_context);

    Ok(script_id)
}

pub fn unload_script(script_id: ScriptId) -> Result<(), Error> {
    if SCRIPT_STORAGE.lock()?.remove(&script_id).is_none() {
        return Err(Error::NoSuchScript);
    }

    Ok(())
}

pub fn run_fn_ptr<T: Clone + Send + Sync + 'static, P: FuncArgs>(
    script_id: ScriptId,
    fn_ptr: &FnPtr,
    args: P,
) -> Result<T, Error> {
    let mut engine = Engine::new_raw();

    RTDBG_PACKAGE.register_into_engine(&mut engine);

    let global_store = GLOBAL_STORE.clone();

    let guard = SCRIPT_STORAGE.lock()?;

    if let Some(script_ctx) = guard.get(&script_id) {
        let script = script_ctx.script.clone();
        let script_store = script_ctx.store.clone();

        drop(guard);

        let mut new_args: Vec<Dynamic> = Vec::new();

        new_args.push(Dynamic::from(script_id));
        new_args.push(Dynamic::from(script_store));
        new_args.push(Dynamic::from(global_store));
        args.parse(&mut new_args);

        return fn_ptr
            .call(&engine, &script.ast, new_args)
            .map_err(|e| e.into());
    }

    Err(Error::NoSuchScript)
}

pub fn run_function<T: Clone + Send + Sync + 'static, P: FuncArgs>(
    script_id: ScriptId,
    function: &str,
    args: P,
) -> Result<T, Error> {
    run_fn_ptr(script_id, &FnPtr::new(function).map_err(Error::from)?, args)
}

pub fn status() -> Result<Vec<ScriptId>, Error> {
    Ok(SCRIPT_STORAGE.lock()?.keys().copied().collect())
}

pub fn rhai_dispatcher(
    hook_id: HookId,
    parameters: Either<&mut Vec<Parameter>, &mut HashMap<String, Parameter>>,
) {
    let Ok(rhai_hook_callbacks_guard) = RHAI_HOOK_CALLBACKS.lock() else {
        warn!("`RHAI_HOOK_CALLBACKS` poisoned, skipping!");
        return;
    };

    let Some(script_callback) = rhai_hook_callbacks_guard.get(&hook_id).cloned() else {
        warn!("No rhai callback associated with {hook_id:?}, skipping!");
        return;
    };

    drop(rhai_hook_callbacks_guard);

    match parameters {
        Either::First(vec) => {
            let vec_cloned = vec.iter().map(|i| Dynamic::from(i.clone())).collect();

            let Ok(return_parameters_vec) = run_fn_ptr::<Array, (Array,)>(
                script_callback.script_id,
                &script_callback.callback,
                (vec_cloned,),
            ) else {
                warn!("Script callback {script_callback:?} did not return an Array of Parameter!");
                return;
            };

            *vec = match return_parameters_vec
                .into_iter()
                .map(|i| i.try_cast::<Parameter>())
                .collect::<Option<Vec<Parameter>>>()
            {
                Some(vec) => vec,
                None => {
                    warn!(
                        "Script callback {script_callback:?} did not return an Array of Parameter!"
                    );
                    return;
                }
            };
        }
        Either::Second(hashmap) => {
            let map_cloned = hashmap
                .iter()
                .map(|(k, v)| (k.into(), Dynamic::from(v.clone())))
                .collect::<Map>();

            let Ok(return_parameters_map) = run_fn_ptr::<Map, (Map,)>(
                script_callback.script_id,
                &script_callback.callback,
                (map_cloned,),
            ) else {
                warn!("Script callback {script_callback:?} did not return a Map!");
                return;
            };

            let Some(return_parameter_hashmap) = return_parameters_map
                .into_iter()
                .map(|(k, v)| Some((k.into(), v.try_cast::<Parameter>()?)))
                .collect::<Option<HashMap<String, Parameter>>>()
            else {
                warn!("Script callback {script_callback:?} did not return a Map of Parameters!");
                return;
            };

            *hashmap = return_parameter_hashmap;
        }
    };
}
