use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    ffi::c_void,
    sync::{LazyLock, Mutex},
};

use libc::{
    _SC_PAGESIZE, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, sysconf,
};
use log::warn;

use crate::{arch::TrampolineGenDesc, bits::Bits, error::Error, parameter::Parameter};

pub static FUNCTIONS: LazyLock<Mutex<HashMap<String, FunctionId>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static HOOKS: LazyLock<Mutex<HashMap<FunctionId, HookManager>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn generate_trampoline<const SIZE: usize>(
    tgd: TrampolineGenDesc<SIZE>,
    function_id: FunctionId,
    original_function: u64,
) -> [u8; SIZE] {
    let function_id = function_id.0.to_ne_bytes();
    #[allow(function_casts_as_integer)]
    let dispatcher_address = (dispatcher as u64).to_ne_bytes();
    let original_function = original_function.to_ne_bytes();

    let mut modified_trampoline = *tgd.trampoline_src;

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

pub struct HookSpace {
    aligned_size: u64,
    page_size: u64,
    pages: VecDeque<(Bits, *mut c_void)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HookId {
    pub id: u64,
    pub associated_fn: FunctionId,
}

#[derive(Debug, Clone, Copy)]
pub enum HookArch {
    X86_64 = 0,
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

impl HookSpace {
    pub fn new(trampoline_size: u64) -> HookSpace {
        // align on 8 byte boundary
        let page_size = unsafe { sysconf(_SC_PAGESIZE) };

        HookSpace {
            aligned_size: trampoline_size.div_ceil(8) * 8,
            page_size: page_size as u64,
            pages: VecDeque::new(),
        }
    }

    pub fn fetch_or_alloc(&mut self) -> Result<*mut u8, Error> {
        let slots = self.page_size / self.aligned_size;

        for (bitmap, ptr) in self.pages.iter_mut() {
            if bitmap.is_all_set() {
                continue;
            }

            for i in 0..slots {
                if bitmap.get(i as usize) == Some(0) {
                    bitmap.set(i as usize, 1);

                    return Ok(unsafe {
                        (*ptr as *mut u8).byte_add((i * self.aligned_size) as usize)
                    });
                }
            }
        }

        self.alloc_page()?;

        self.fetch_or_alloc()
    }

    pub fn alloc_page(&mut self) -> Result<(), Error> {
        let slots = self.page_size / self.aligned_size;

        let address = unsafe {
            mmap(
                std::ptr::null_mut(),
                self.page_size as usize,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        self.pages.push_front((Bits::new(slots as usize), address));
        Ok(())
    }

    pub fn free(&mut self, ptr: *mut u8) {
        let ptr = ptr as u64;

        for (bitmap, ptr_s) in self.pages.iter_mut() {
            let ptr_base = *ptr_s as u64;

            if ptr_base <= ptr && ptr < ptr_base + self.page_size {
                // If we're here then this is the page that contains that particular pointer
                let slot = (ptr - ptr_base) / self.aligned_size;

                bitmap.set(slot as usize, 0);
            }
        }
    }
}

impl TryFrom<u64> for HookArch {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HookArch::X86_64),
            _ => Err(Error::UnknownArchitecture(value)),
        }
    }
}

thread_local! {
    static RETURN_ADRESSES: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
}

extern "C" fn dispatcher(
    function_id: u64,
    is_prefix: bool,
    register_snapshot: u64,
    original_return: u64,
    return_slot_address: u64,
    hook_arch: u64,
) -> u64 {
    let original_return = if is_prefix {
        RETURN_ADRESSES.with_borrow_mut(|v| v.insert(return_slot_address, original_return));
        0
    } else {
        RETURN_ADRESSES.with_borrow_mut(|v| {
            *v.get(&return_slot_address).unwrap_or_else(|| {
                warn!("[HOOK DISPATCHER] Postfix hook hit but no return address available!");
                &0
            })
        })
    };

    let Ok(hook_arch) = HookArch::try_from(hook_arch) else {
        warn!("[HOOK DISPATCHER] Unknown architecture!");
        return original_return;
    };

    let reg_snapshot = unsafe {
        match hook_arch {
            HookArch::X86_64 => {
                &mut *(register_snapshot as *mut crate::arch::amd64::Amd64RegisterSnapshot)
            }
        }
    };

    let stack_base = register_snapshot + size_of_val(reg_snapshot) as u64;

    let function_id = FunctionId(function_id);

    let hm_guard = match HOOKS.lock() {
        Ok(g) => g,
        Err(e) => {
            warn!("[HOOK DISPATCHER] Hook resolution error: {e}");
            return original_return;
        }
    };

    let hook_manager = match hm_guard.get(&function_id) {
        Some(hm) => hm,
        None => {
            warn!("[HOOK DISPATCHER] Illegal hook state!");
            return original_return;
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

    hooks.sort_unstable_by_key(|a| a.1.priority);

    'hooks: for (id, mut ctx) in hooks {
        if let HookStatus::Inactive = ctx.status {
            continue;
        }

        for parameter in ctx.parameters.values_mut() {
            if let Err(e) = parameter.try_resolve(reg_snapshot, stack_base) {
                warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                continue 'hooks;
            }
        }

        (ctx.callback)(id, &mut ctx.parameters);

        for parameter in ctx.parameters.values() {
            if let Err(e) = parameter.try_commit(reg_snapshot, stack_base) {
                warn!(
                    "[HOOK DISPATCHER] Parameter {parameter:?} of hook {id:?} encountered error: {e}"
                );
            }
        }
    }

    original_return
}
