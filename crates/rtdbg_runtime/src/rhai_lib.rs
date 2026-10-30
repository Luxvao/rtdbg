use std::{collections::HashMap, sync::LazyLock};

use imep::{
    ElfHeader, ElfRel, ElfRela, ElfSym, ProgramHeader, ProgramType, RelocType, rhai_enum_impl::*,
};
use imp_core::{ImportSlot, Module, ObjectFormat, Parser, rhai_enum_impl::*};

use librtdbg::{
    arch::amd64::Amd64RegisterSnapshot,
    either::Either,
    hook::{
        self, CallControl, HookId, HookOrder, HookPhase, HookStatus, ImportId, attach_hook,
        hookphase_module, hookstatus_module, patch_import,
    },
    os::{
        NATIVE_PARSER_EXTRA, NativeParser,
        linux::{linux_module, process::LinuxProcess},
    },
    parameter::{
        Parameter, ParameterLocation, ParameterValue, parameter_module, parameterlocation_module,
        parametervalue_module,
    },
    register::{Register, register_module},
    register_modules, register_types,
    script::{ScriptCallback, ScriptId, Store},
};
use rhai::{
    Array, Dynamic, Engine, EvalAltResult, FnPtr, FuncRegistration, Map, def_package,
    exported_module, packages::StandardPackage,
};

use crate::runtime::{RHAI_HOOK_CALLBACKS, rhai_dispatcher};

pub static RTDBG_PACKAGE: LazyLock<RtdbgPackage> = LazyLock::new(RtdbgPackage::new);

def_package! {
    pub RtdbgPackage(module) : StandardPackage {
        // Functions
        FuncRegistration::new("read_mem").set_into_module(module, read_mem);
        FuncRegistration::new("write_mem").set_into_module(module, write_mem_arr);
        FuncRegistration::new("write_mem").set_into_module(module, write_mem_string);

        // Module
        FuncRegistration::new("modules").set_into_module(module, modules);

        // Import slots
        FuncRegistration::new("import_slots").set_into_module(module, import_slots);
        FuncRegistration::new("find_import_slots").set_into_module(module, find_import_slots_module);
        FuncRegistration::new("find_import_slots").set_into_module(module, find_import_slots_module_path);

        // Hook status
        FuncRegistration::new("set_hook_status").set_into_module(module, set_hook_status);

        // Hook registration
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_id_vec);
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_id_map);
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_slot_vec);
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_slot_map);
        FuncRegistration::new("detach_hook").set_into_module(module, detach_hook);

        #[cfg(target_os = "linux")]
        module.set_var("OS", "Linux");

        // Constants
    } |> |engine| {
        engine.on_print(|s| println!("{s}"));
        engine.on_debug(|s, source, pos| {
            println!("{source:?} @ {pos:?}: {s}");
        });

        setup_types(engine);
        setup_modules(engine);
    }
}

pub fn setup_types(engine: &mut Engine) {
    register_types!(engine, {
        // ELF structures
        ElfHeader,
        ProgramHeader,
        ElfSym,
        ElfRel,
        ElfRela,

        // Linux
        LinuxProcess,

        // Import handling and modules
        ImportSlot,
        ImportId,
        Module,

        // Hooking
        HookId,
        HookOrder,
        CallControl,
        Parameter,

        // Runtime state
        Store,
        Amd64RegisterSnapshot
    });
}

pub fn setup_modules(engine: &mut Engine) {
    #[cfg(target_os = "linux")]
    linux_module(engine);

    register_modules!(engine, {
        // Generic parsing structures
        ObjectFormat,

        // Program headers and relocations
        ProgramType,
        RelocType,

        // CPU state
        Register,

        // Hooking
        HookPhase,
        HookStatus,

        // Parameter system
        Parameter,
        ParameterValue,
        ParameterLocation
    });
}

// Reading a specific amount of data from an address into an array
fn read_mem(addy: i64, size: i64) -> Vec<u8> {
    let addy = addy as *const u8;
    let size = size as usize;

    let mut output = Vec::new();

    for i in 0..size {
        unsafe {
            output.push(*(addy.add(i)));
        }
    }

    output
}

// Writing an array to a specific address
fn write_mem_arr(addy: i64, new: Vec<u8>) {
    let addy = addy as *mut u8;

    for (i, byte) in new.iter().enumerate() {
        unsafe {
            *(addy.add(i)) = *byte;
        }
    }
}

// Writing a string (null terminated) to a specific address
fn write_mem_string(addy: i64, new: String) {
    let mut new = new.into_bytes();

    new.push(b'\0');

    write_mem_arr(addy, new.into());
}

// Get all modules
fn modules() -> Dynamic {
    Dynamic::from_array(
        NativeParser::modules(NATIVE_PARSER_EXTRA)
            .map(|i| Dynamic::from(i))
            .collect(),
    )
}

// Get every import slot for all modules to then match on
fn import_slots() -> Dynamic {
    Dynamic::from_array(
        NativeParser::import_slots(NATIVE_PARSER_EXTRA)
            .map(|i| Dynamic::from(i))
            .collect(),
    )
}

// Return a vector of import slots matching module paths and import names
fn find_import_slots_module_path(module_path: String, import_name: String) -> Dynamic {
    Dynamic::from_array(
        NativeParser::import_slots(NATIVE_PARSER_EXTRA)
            .filter(|i| {
                i.module_path.as_deref() == Some(&module_path) && i.import_name == import_name
            })
            .map(|i| Dynamic::from(i))
            .collect(),
    )
}

// Return a vector of import slots matching module paths and import names (taking a Module struct instead)
fn find_import_slots_module(module: Module, import_name: String) -> Dynamic {
    Dynamic::from_array(
        NativeParser::import_slots(NATIVE_PARSER_EXTRA)
            .filter(|i| i.module_path == module.path && i.import_name == import_name)
            .map(|i| Dynamic::from(i))
            .collect(),
    )
}

// Translate an ImportSlot into an ImportId
fn get_import_id(import_slot: ImportSlot) -> Result<ImportId, Box<EvalAltResult>> {
    if let Some(id) = hook::get_import_id(&import_slot).map_err(|e| e.to_string())? {
        return Ok(id);
    }

    patch_import(None, import_slot).map_err(|e| e.to_string().into())
}

// Generic ImportId-based attach; internal, called by dedicated functions for the Either variants
fn attach_hook_import_id_generic(
    import_id: ImportId,
    phase: HookPhase,
    status: HookStatus,
    priority: HookOrder,
    parameters: Either<Vec<Parameter>, HashMap<String, Parameter>>,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    let script_callback = ScriptCallback {
        script_id,
        callback,
    };

    // We register it inactive to make sure it's not actually hit untill we set up the glue
    let hook_id = attach_hook(
        import_id,
        phase,
        HookStatus::Inactive,
        priority,
        parameters,
        rhai_dispatcher,
    )
    .map_err(|e| e.to_string())?;

    // Now we actually make our runtime aware of the association
    RHAI_HOOK_CALLBACKS
        .lock()
        .map_err(|e| e.to_string())?
        .insert(hook_id, script_callback);

    // If they want it active, we set it to active
    if let HookStatus::Active = status {
        set_hook_status(hook_id, HookStatus::Active).map_err(|e| e.to_string())?;
    }

    Ok(hook_id)
}

// Generic ImportSlot-based attach. It automatically patches if needed, so it's a single call
fn attach_hook_import_slot_generic(
    import_slot: ImportSlot,
    phase: HookPhase,
    status: HookStatus,
    priority: HookOrder,
    parameters: Either<Vec<Parameter>, HashMap<String, Parameter>>,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    let import_id = get_import_id(import_slot)?;

    attach_hook_import_id_generic(
        import_id, phase, status, priority, parameters, script_id, callback,
    )
}

fn attach_hook_import_id_vec(
    import_id: ImportId,
    phase: HookPhase,
    status: HookStatus,
    priority: HookOrder,
    parameters: Array,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_id_generic(
        import_id,
        phase,
        status,
        priority,
        Either::First(
            parameters
                .into_iter()
                .map(|i| Some(i.try_cast::<Parameter>()?))
                .collect::<Option<Vec<Parameter>>>()
                .ok_or("Parameters must be an array of type Parameter!")?,
        ),
        script_id,
        callback,
    )
}

fn attach_hook_import_id_map(
    import_id: ImportId,
    phase: HookPhase,
    status: HookStatus,
    priority: HookOrder,
    parameters: Map,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_id_generic(
        import_id,
        phase,
        status,
        priority,
        Either::Second(
            parameters
                .into_iter()
                .map(|(k, v)| Some((k.to_string(), v.try_cast::<Parameter>()?)))
                .collect::<Option<HashMap<String, Parameter>>>()
                .ok_or("Parameters must be a Map of Parameter!")?,
        ),
        script_id,
        callback,
    )
}

fn attach_hook_import_slot_vec(
    import_slot: ImportSlot,
    phase: HookPhase,
    status: HookStatus,
    priority: HookOrder,
    parameters: Array,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_slot_generic(
        import_slot,
        phase,
        status,
        priority,
        Either::First(
            parameters
                .into_iter()
                .map(|i| Some(i.try_cast::<Parameter>()?))
                .collect::<Option<Vec<Parameter>>>()
                .ok_or("Parameters must be an array of type Parameter!")?,
        ),
        script_id,
        callback,
    )
}

fn attach_hook_import_slot_map(
    import_slot: ImportSlot,
    phase: HookPhase,
    status: HookStatus,
    priority: HookOrder,
    parameters: Map,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_slot_generic(
        import_slot,
        phase,
        status,
        priority,
        Either::Second(
            parameters
                .into_iter()
                .map(|(k, v)| Some((k.to_string(), v.try_cast::<Parameter>()?)))
                .collect::<Option<HashMap<String, Parameter>>>()
                .ok_or("Parameters must be a Map of Parameter!")?,
        ),
        script_id,
        callback,
    )
}

// Helper to detach a hook completely
fn detach_hook(hook_id: HookId) -> Result<(), Box<EvalAltResult>> {
    hook::detach_hook(hook_id).map_err(|e| e.to_string())?;

    RHAI_HOOK_CALLBACKS
        .lock()
        .map_err(|e| e.to_string())?
        .remove(&hook_id);

    Ok(())
}

// Helper for setting the status of a hook
fn set_hook_status(hook_id: HookId, status: HookStatus) -> Result<(), Box<EvalAltResult>> {
    hook::set_hook_status(hook_id, status).map_err(|e| e.to_string())?;

    Ok(())
}
