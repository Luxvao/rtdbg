use std::{collections::HashMap, os::raw::c_void, sync::LazyLock};

use imep::{
    Class, ElfHeader, ElfRel, ElfRela, ElfSym, ElfType, Endianness, Machine, OsAbi, ProgramHeader,
    ProgramType, RelocType,
    enum_rhai_impl::*,
    parser::{ElfParser, Libc, ParseInfo},
};
use imp_core::{ImportSlot, Parser};
use paste::paste;

use librtdbg::{
    arch::amd64::Amd64RegisterSnapshot,
    either::Either,
    hook::{
        self, HookId, HookPriority, HookStatus, ImportId, attach_hook, hookstatus_module,
        patch_import,
    },
    parameter::{
        Parameter, ParameterLocation, ParameterValue, parameter_module, parameterlocation_module,
        parametervalue_module,
    },
    proc_utils::{Permissions, Process, Vma, Vmas},
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
        FuncRegistration::new("get_proc_info").set_into_module(module, get_proc_info);
        FuncRegistration::new("get_elf_header").set_into_module(module, get_elf_header);
        FuncRegistration::new("get_vmas").set_into_module(module, get_vmas);
        FuncRegistration::new("read_mem").set_into_module(module, read_mem);
        FuncRegistration::new("write_mem").set_into_module(module, write_mem_arr);
        FuncRegistration::new("write_mem").set_into_module(module, write_mem_string);
        FuncRegistration::new("mprotect").set_into_module(module, mprotect_rhai);

        // Hooks and stuff
        FuncRegistration::new("get_import_slots").set_into_module(module, get_import_slots);
        FuncRegistration::new("set_hook_status").set_into_module(module, set_hook_status);
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_id_vec);
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_id_map);
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_slot_vec);
        FuncRegistration::new("attach_hook").set_into_module(module, attach_hook_import_slot_map);
        FuncRegistration::new("detach_hook").set_into_module(module, detach_hook);

        // Constants
        module.set_var("PROT_NONE", libc::PROT_NONE);
        module.set_var("PROT_READ", libc::PROT_READ);
        module.set_var("PROT_WRITE", libc::PROT_WRITE);
        module.set_var("PROT_EXEC", libc::PROT_EXEC);
        module.set_var("PROT_GROWSUP", libc::PROT_GROWSUP);
        module.set_var("PROT_GROWSDOWN", libc::PROT_GROWSDOWN);
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
    register_types!(engine, { Permissions, Vma, Vmas, Process, ImportSlot, ElfHeader, ProgramHeader, ElfSym, ElfRel, ElfRela, Store, Amd64RegisterSnapshot, ImportId, HookId, HookPriority, Parameter });
}

pub fn setup_modules(engine: &mut Engine) {
    register_modules!(engine, { Class, Endianness, OsAbi, ElfType, Machine, ProgramType, RelocType, Register, HookStatus, ParameterValue, ParameterLocation, Parameter });
}

// Get process info
fn get_proc_info() -> Result<Process, Box<EvalAltResult>> {
    Process::this().map_err(|e| format!("{e}").into())
}

// Get maps info
fn get_vmas() -> Result<Vmas, Box<EvalAltResult>> {
    Vmas::this().map_err(|e| format!("{e}").into())
}

// Get ELF header
fn get_elf_header() -> Result<ElfHeader, Box<EvalAltResult>> {
    let proc_info = get_proc_info()?;

    let header_address = proc_info
        .vmas
        .iter()
        .find(|vma| vma.path.eq(&Some(proc_info.path.clone())) && vma.offset == 0)
        .ok_or("Unable to find ELF header")?
        .saddy;

    Ok(unsafe { ElfHeader::parse_at(header_address).map_err(|_| "Unable to parse ELF header")? })
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

// Writing a string to a specific address
fn write_mem_string(addy: i64, new: String) {
    let new = new.as_bytes();

    write_mem_arr(addy, new.into());
}

// Mprotect wrapped for rhai - handles page boundaries itself
fn mprotect_rhai(addy: i64, size: i64, prot: i32) -> i64 {
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };

    let offset = addy % page_size;

    let addy_aligned = addy - offset;

    let size_with_adjustments = size + offset;

    unsafe {
        i64::from(libc::mprotect(
            addy_aligned as *mut c_void,
            size_with_adjustments as usize,
            prot,
        ))
    }
}

// Get every import slot for all modules to then match on
fn get_import_slots() -> Dynamic {
    // Hardcode imep for now
    let parse_info = ParseInfo::new(
        Libc::COMPILED_WITH,
        Class::NATIVE,
        Endianness::NATIVE,
        Machine::NATIVE,
    );

    Dynamic::from_array(
        ElfParser::import_slots(parse_info)
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
    prefix: bool,
    status: HookStatus,
    priority: HookPriority,
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
        prefix,
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
    prefix: bool,
    status: HookStatus,
    priority: HookPriority,
    parameters: Either<Vec<Parameter>, HashMap<String, Parameter>>,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    let import_id = get_import_id(import_slot)?;

    attach_hook_import_id_generic(
        import_id, prefix, status, priority, parameters, script_id, callback,
    )
}

fn attach_hook_import_id_vec(
    import_id: ImportId,
    prefix: bool,
    status: HookStatus,
    priority: HookPriority,
    parameters: Array,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_id_generic(
        import_id,
        prefix,
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
    prefix: bool,
    status: HookStatus,
    priority: HookPriority,
    parameters: Map,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_id_generic(
        import_id,
        prefix,
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
    prefix: bool,
    status: HookStatus,
    priority: HookPriority,
    parameters: Array,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_slot_generic(
        import_slot,
        prefix,
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
    prefix: bool,
    status: HookStatus,
    priority: HookPriority,
    parameters: Map,
    script_id: ScriptId,
    callback: FnPtr,
) -> Result<HookId, Box<EvalAltResult>> {
    attach_hook_import_slot_generic(
        import_slot,
        prefix,
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
