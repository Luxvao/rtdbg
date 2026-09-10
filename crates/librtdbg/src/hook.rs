use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, atomic::AtomicU64},
};

use log::warn;

use crate::{arch::TrampolineGenDesc, parameter::Parameter};

pub static FUNCTIONS: LazyLock<Mutex<HashMap<String, FunctionId>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static HOOKS: LazyLock<Mutex<HashMap<FunctionId, HookManager>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub fn generate_trampoline<const SIZE: usize>(
    tgd: TrampolineGenDesc<SIZE>,
    function_id: FunctionId,
    original_function: u64,
) -> [u8; SIZE] {
    let function_id = function_id.0.to_ne_bytes();
    #[allow(function_casts_as_integer)]
    let dispatcher_address = (dispatcher as u64).to_ne_bytes();
    let original_function = original_function.to_ne_bytes();

    let mut modified_trampoline = tgd.trampoline_src;

    for (i, b) in function_id.iter().enumerate() {
        modified_trampoline[tgd.function_id_offset as usize + i] = *b;
    }

    for (i, b) in dispatcher_address.iter().enumerate() {
        modified_trampoline[tgd.dispatcher_address_offset as usize + i] = *b;
    }

    for (i, b) in original_function.iter().enumerate() {
        modified_trampoline[tgd.original_function_address_offset as usize + i] = *b;
    }

    modified_trampoline
}

pub struct HookSpace {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HookId {
    pub id: u64,
    pub associated_fn: FunctionId,
}

#[derive(Debug, Clone)]
pub struct HookManager {
    pub prefix: HashMap<HookId, HookCtx>,
    pub postfix: HashMap<HookId, HookCtx>,
}

#[derive(Debug, Clone)]
pub struct HookCtx {
    pub status: HookStatus,
    pub priority: HookPriority,
    pub parameters: HashMap<String, Parameter>,
    pub callback: fn(HookId, &mut HashMap<String, Parameter>),
}

#[derive(Debug, Clone, Copy)]
pub enum HookStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct HookPriority(u64);

extern "C" fn dispatcher(function_id: u64, is_prefix: bool, register_snapshot: u64) {
    #[cfg(target_arch = "x86_64")]
    let reg_snapshot =
        unsafe { &mut *(register_snapshot as *mut crate::arch::amd64::Amd64RegisterSnapshot) };

    let stack_base = register_snapshot + size_of_val(reg_snapshot) as u64;

    let function_id = FunctionId(function_id);

    let hm_guard = match HOOKS.lock() {
        Ok(g) => g,
        Err(e) => {
            warn!("Hook resolution error: {e}");
            return;
        }
    };

    let hook_manager = match hm_guard.get(&function_id) {
        Some(hm) => hm,
        None => {
            warn!("Illegal hook state!");
            return;
        }
    };

    let mut hooks = if is_prefix {
        hook_manager
            .prefix
            .iter()
            .map(|(id, ctx)| (*id, ctx.clone()))
            .collect::<Vec<(HookId, HookCtx)>>()
    } else {
        hook_manager
            .postfix
            .iter()
            .map(|(id, ctx)| (*id, ctx.clone()))
            .collect::<Vec<(HookId, HookCtx)>>()
    };

    drop(hm_guard);

    hooks.sort_unstable_by(|a, b| a.1.priority.cmp(&b.1.priority));

    'hooks: for (id, mut ctx) in hooks {
        if let HookStatus::Inactive = ctx.status {
            continue;
        }

        for parameter in ctx.parameters.values_mut() {
            if let Err(e) = parameter.try_resolve(reg_snapshot, stack_base) {
                warn!("Hook {id:?} encountered error: {e}");
                continue 'hooks;
            }
        }

        (ctx.callback)(id, &mut ctx.parameters);

        for parameter in ctx.parameters.values() {
            if let Err(e) = parameter.try_commit(reg_snapshot, stack_base) {
                warn!("Parameter {parameter:?} of hook {id:?} encountered error: {e}");
                continue;
            }
        }
    }
}
