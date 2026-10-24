use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    ptr,
    sync::{
        LazyLock, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use imp_core::ImportSlot;
use libc::{
    _SC_PAGESIZE, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE, mmap, sysconf,
};
use log::warn;
use rhai::{CustomType, TypeBuilder};

use crate::{
    ID_GEN,
    arch::{self, TrampolineGenDesc},
    bits::Bits,
    create_enum_module,
    either::Either,
    error::Error,
    parameter::Parameter,
};

pub static TRAMPOLINE_ALLOCATOR: LazyLock<Mutex<TrampolineAllocator>> =
    LazyLock::new(|| Mutex::new(TrampolineAllocator::new(arch::NATIVE_TRAMPOLINE_SIZE)));

pub static PATCHES: LazyLock<Mutex<HashMap<ImportSlot, ImportCtx>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static HOOKS: LazyLock<Mutex<HashMap<ImportId, HookManager>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn generate_trampoline<const SIZE: usize>(
    tgd: TrampolineGenDesc<SIZE>,
    import_id: ImportId,
    original_function: u64,
) -> [u8; SIZE] {
    let function_id = import_id.0.to_ne_bytes();
    #[allow(function_casts_as_integer)]
    let dispatcher_address = (dispatcher as usize).to_ne_bytes();
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

fn free_trampoline_and_err(trampoline_ptr: *mut u8, err: Error) -> Error {
    match TRAMPOLINE_ALLOCATOR.lock() {
        Ok(mut alloc) => {
            if let Err(e) = alloc.free(trampoline_ptr) {
                log::warn!("failed to free trampoline: {e:?}");
            }
        }
        Err(_) => log::warn!("failed to lock allocator to free trampoline"),
    }
    err
}

#[derive(Debug)]
pub struct TrampolineAllocator {
    aligned_size: u64,
    page_size: u64,
    pages: VecDeque<(Bits, u64)>,
}

#[derive(Debug, Clone, Copy)]
pub struct ImportCtx {
    trampoline_ptr: u64,
    old_ptr: u64,
    import_id: ImportId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CustomType)]
pub struct ImportId(u64);

pub type HookCallback = fn(HookId, Either<&mut Vec<Parameter>, &mut HashMap<String, Parameter>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CustomType)]
pub struct HookId {
    #[rhai_type(readonly)]
    pub id: u64,
    #[rhai_type(readonly)]
    pub associated_import: ImportId,
}

#[derive(Default, Debug, Clone)]
pub struct HookManager {
    pub prefix: Vec<HookCtx>,
    pub postfix: Vec<HookCtx>,
}

#[derive(Debug, Clone)]
pub struct HookCtx {
    pub id: HookId,
    pub status: HookStatus,
    pub priority: HookPriority,
    pub parameters: Either<Vec<Parameter>, HashMap<String, Parameter>>,
    pub callback: HookCallback,
}

#[derive(Debug, Clone, Copy)]
pub enum HookStatus {
    Active,
    Inactive,
}

create_enum_module!(HookStatus => Active, Inactive);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct HookPriority(u64);

impl TrampolineAllocator {
    pub fn new(trampoline_size: u64) -> TrampolineAllocator {
        // align on 8 byte boundary
        let page_size = unsafe { sysconf(_SC_PAGESIZE) };

        TrampolineAllocator {
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
                    bitmap
                        .set(i as usize, 1)
                        .map_err(|_| Error::UnableToAllocateHook)?;

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

        if address as i64 == -1 {
            return Err(Error::UnableToAllocatePage);
        }

        self.pages
            .push_front((Bits::new(slots as usize), address as u64));
        Ok(())
    }

    pub fn free(&mut self, ptr: *mut u8) -> Result<(), Error> {
        let ptr = ptr as u64;

        for (bitmap, ptr_s) in self.pages.iter_mut() {
            let ptr_base = *ptr_s;

            if ptr_base <= ptr && ptr < ptr_base + self.page_size {
                // If we're here then this is the page that contains that particular pointer
                let slot = (ptr - ptr_base) / self.aligned_size;

                bitmap
                    .set(slot as usize, 0)
                    .map_err(|_| Error::UnableToFreeHook)?;
            }
        }

        Ok(())
    }
}

impl HookPriority {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.with_fn("HookPriority", |v: i64| HookPriority(v as u64));
    }
}

thread_local! {
    static RETURN_ADRESSES: RefCell<HashMap<u64, u64>> = RefCell::new(HashMap::new());
}

extern "C" fn dispatcher(
    import_id: u64,
    is_prefix: bool,
    register_snapshot: u64,
    original_return: u64,
    return_slot_address: u64,
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

    let reg_snapshot = unsafe { &mut *(register_snapshot as *mut arch::NativeRegisterStruct) };

    let stack_base = register_snapshot + size_of_val(reg_snapshot) as u64;

    let import_id = ImportId(import_id);

    let hm_guard = match HOOKS.lock() {
        Ok(g) => g,
        Err(e) => {
            warn!("[HOOK DISPATCHER] Hook resolution error: {e}");
            return original_return;
        }
    };

    let hook_manager = match hm_guard.get(&import_id) {
        Some(hm) => hm,
        None => {
            warn!("[HOOK DISPATCHER] Illegal hook state!");
            return original_return;
        }
    };

    let mut hooks = if is_prefix {
        hook_manager.prefix.clone()
    } else {
        hook_manager.postfix.clone()
    };

    drop(hm_guard);

    'hooks: for ctx in hooks.iter_mut() {
        if let HookStatus::Inactive = ctx.status {
            continue;
        }

        let id = ctx.id;

        match &mut ctx.parameters {
            Either::First(vec) => {
                for parameter in vec.iter_mut() {
                    if let Err(e) = parameter.try_resolve(reg_snapshot, stack_base) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
            Either::Second(map) => {
                for parameter in map.values_mut() {
                    if let Err(e) = parameter.try_resolve(reg_snapshot, stack_base) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
        }

        (ctx.callback)(id, ctx.parameters.as_mut());

        match &ctx.parameters {
            Either::First(vec) => {
                for parameter in vec.iter() {
                    if let Err(e) = parameter.try_commit(reg_snapshot, stack_base) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
            Either::Second(map) => {
                for parameter in map.values() {
                    if let Err(e) = parameter.try_commit(reg_snapshot, stack_base) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
        }
    }

    original_return
}

pub fn get_import_id(import_slot: &ImportSlot) -> Result<Option<ImportId>, Error> {
    Ok(PATCHES.lock()?.get(import_slot).map(|i| i.import_id))
}

// Patch import entry
pub fn patch_import(
    existing_import_id: Option<ImportId>,
    import_slot: ImportSlot,
) -> Result<ImportId, Error> {
    // TODO: lazy binding can overwrite this slot

    // We don't want to allow mutation while we're changing it
    let mut patches_lock = PATCHES.lock()?;

    if patches_lock.contains_key(&import_slot) {
        return Err(Error::ImportAlreadyPatched(import_slot));
    }

    // If we have an existing import_id, we can skip generating a new one
    let import_id = match existing_import_id {
        Some(id) => id,
        None => ImportId(ID_GEN.fetch_add(1, Ordering::Relaxed)),
    };

    let slot = unsafe { AtomicUsize::from_ptr(import_slot.ptr as *mut usize) };

    let original_address = slot.load(Ordering::Acquire);

    // Generating a native trampoline
    let trampoline = generate_trampoline(
        arch::NATIVE_TRAMPOLINE_GEN_DESC,
        import_id,
        original_address as u64,
    );

    let trampoline_ptr = TRAMPOLINE_ALLOCATOR.lock()?.fetch_or_alloc()?;

    unsafe {
        // Here we write the trampoline into the allocated space
        ptr::copy_nonoverlapping(trampoline.as_ptr(), trampoline_ptr, trampoline.len());
        if !clear_cache::clear_cache(trampoline_ptr, trampoline_ptr.byte_add(trampoline.len())) {
            // Critical error, couldn't clear cache
            return Err(free_trampoline_and_err(
                trampoline_ptr,
                Error::IcacheFlushFailed,
            ));
        }
    }

    // Here we swap the pointer if it hasn't changed in the meantime
    slot.compare_exchange(
        original_address,
        trampoline_ptr as usize,
        Ordering::Release,
        Ordering::Relaxed,
    )
    .map_err(|_| {
        free_trampoline_and_err(trampoline_ptr, Error::SlotChanged(import_slot.clone()))
    })?;

    patches_lock.insert(
        import_slot,
        ImportCtx {
            trampoline_ptr: trampoline_ptr as u64,
            old_ptr: original_address as u64,
            import_id,
        },
    );

    Ok(import_id)
}

pub fn unpatch_import(import_slot: ImportSlot) -> Result<ImportId, Error> {
    let mut patches_lock = PATCHES.lock()?;

    // If it isn't in PATCHES, then as far as we know, it's not patched
    let import_ctx = match patches_lock.remove(&import_slot) {
        Some(ctx) => ctx,
        None => return Err(Error::ImportNotPatched(import_slot)),
    };

    let slot = unsafe { AtomicUsize::from_ptr(import_slot.ptr as *mut usize) };

    // If it's not pointing at our trampoline, then something has interfered and we shouldn't do anything but deallocate the trampoline
    slot.compare_exchange(
        import_ctx.trampoline_ptr as usize,
        import_ctx.old_ptr as usize,
        Ordering::Release,
        Ordering::Relaxed,
    )
    .map_err(|_| {
        free_trampoline_and_err(
            import_ctx.trampoline_ptr as *mut u8,
            Error::SlotChanged(import_slot),
        )
    })?;

    TRAMPOLINE_ALLOCATOR
        .lock()?
        .free(import_ctx.trampoline_ptr as *mut u8)?;

    Ok(import_ctx.import_id)
}

pub fn attach_hook(
    import_id: ImportId,
    prefix: bool,
    status: HookStatus,
    priority: HookPriority,
    parameters: Either<Vec<Parameter>, HashMap<String, Parameter>>,
    callback: HookCallback,
) -> Result<HookId, Error> {
    let mut hooks_guard = HOOKS.lock()?;

    let hook_mgr = hooks_guard
        .entry(import_id)
        .or_insert(HookManager::default());

    let hook_id = HookId {
        id: ID_GEN.fetch_add(1, Ordering::Relaxed),
        associated_import: import_id,
    };

    let list = if prefix {
        &mut hook_mgr.prefix
    } else {
        &mut hook_mgr.postfix
    };

    let pos = list.partition_point(|existing| existing.priority <= priority);

    list.insert(
        pos,
        HookCtx {
            id: hook_id,
            status,
            priority,
            parameters,
            callback,
        },
    );

    Ok(hook_id)
}

pub fn detach_hook(hook_id: HookId) -> Result<(), Error> {
    let mut hooks_guard = HOOKS.lock()?;

    let Some(hook_mgr) = hooks_guard.get_mut(&hook_id.associated_import) else {
        return Ok(());
    };

    if let Some(index) = hook_mgr.prefix.iter().position(|i| i.id == hook_id) {
        hook_mgr.prefix.remove(index);

        return Ok(());
    }

    if let Some(index) = hook_mgr.postfix.iter().position(|i| i.id == hook_id) {
        hook_mgr.prefix.remove(index);
    }

    Ok(())
}

pub fn set_hook_status(hook_id: HookId, status: HookStatus) -> Result<(), Error> {
    let mut hooks_guard = HOOKS.lock()?;

    let Some(hook_mgr) = hooks_guard.get_mut(&hook_id.associated_import) else {
        return Err(Error::NoSuchHook(hook_id));
    };

    for prefix_hook in hook_mgr.prefix.iter_mut() {
        if prefix_hook.id == hook_id {
            prefix_hook.status = status;
            return Ok(());
        }
    }

    for postfix_hook in hook_mgr.postfix.iter_mut() {
        if postfix_hook.id == hook_id {
            postfix_hook.status = status;
            return Ok(());
        }
    }

    Err(Error::NoSuchHook(hook_id))
}
