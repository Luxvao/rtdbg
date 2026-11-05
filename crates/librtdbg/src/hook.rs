use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, VecDeque},
    ptr,
    sync::{
        Arc, LazyLock, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use imp_core::ImportSlot;
use log::warn;
use rhai::{CustomType, EvalAltResult, TypeBuilder};

use crate::{
    ID_GEN,
    arch::{self, TrampolineGenDesc},
    bits::Bits,
    create_enum_module,
    either::Either,
    error::Error,
    os::{
        NativeMemoryProvider,
        memory::{LocationHint, MemoryPage, MemoryProvider, PageProtections},
    },
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
        modified_trampoline[tgd.import_id_offset as usize + i] = *b;
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
    pages: VecDeque<(Bits, MemoryPage)>,
}

#[derive(Debug, Clone, Copy)]
pub struct ImportCtx {
    trampoline_ptr: u64,
    old_ptr: u64,
    import_id: ImportId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CustomType)]
pub struct ImportId(u64);

pub type HookCallback =
    fn(HookId, &mut Either<Vec<Parameter>, HashMap<String, Parameter>>, Option<&mut CallControl>);

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
    pub priority: HookOrder,
    pub parameters: Either<Vec<Parameter>, HashMap<String, Parameter>>,
    pub callback: HookCallback,
}

#[derive(Debug, Clone, Copy)]
pub enum HookPhase {
    Prefix,
    Postfix,
}

create_enum_module!(HookPhase => Prefix, Postfix);

#[derive(Debug, Clone, Copy)]
pub enum HookStatus {
    Active,
    Inactive,
}

create_enum_module!(HookStatus => Active, Inactive);

#[derive(Default, Debug, Clone, Copy, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct CallControl {
    cancelled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct HookOrder(i64);

impl TrampolineAllocator {
    pub fn new(trampoline_size: u64) -> TrampolineAllocator {
        // align on 8 byte boundary
        let page_size = NativeMemoryProvider::page_size();

        TrampolineAllocator {
            aligned_size: trampoline_size.div_ceil(8) * 8,
            page_size: page_size as u64,
            pages: VecDeque::new(),
        }
    }

    pub fn fetch_or_alloc(&mut self) -> Result<(&mut MemoryPage, usize), Error> {
        let slots = self.page_size / self.aligned_size;

        for (bitmap, page) in self.pages.iter_mut() {
            if bitmap.is_all_set() {
                continue;
            }

            for i in 0..slots {
                if bitmap.get(i as usize) == Some(0) {
                    bitmap
                        .set(i as usize, 1)
                        .map_err(|_| Error::UnableToAllocateHook)?;

                    return Ok((page, i as usize * self.aligned_size as usize));
                }
            }
        }

        self.alloc_page()?;

        self.fetch_or_alloc()
    }

    pub fn alloc_page(&mut self) -> Result<(), Error> {
        let slots = self.page_size / self.aligned_size;

        let page = NativeMemoryProvider::allocate_page(LocationHint::None, PageProtections::RX)?;

        self.pages.push_front((Bits::new(slots as usize), page));
        Ok(())
    }

    pub fn free(&mut self, ptr: *mut u8) -> Result<(), Error> {
        let ptr = ptr as u64;

        for (bitmap, page) in self.pages.iter_mut() {
            if page.get_ptr() as u64 <= ptr && ptr < page.get_ptr() as u64 + self.page_size {
                // If we're here then this is the page that contains that particular pointer
                let slot = (ptr - page.get_ptr() as u64) / self.aligned_size;

                bitmap
                    .set(slot as usize, 0)
                    .map_err(|_| Error::UnableToFreeHook)?;
            }
        }

        Ok(())
    }
}

impl HookOrder {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.with_fn("HookOrder", |v: i64| HookOrder(v));
    }
}

impl CallControl {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_fn("proceed", Self::proceed_rhai)
            .with_fn("cancel", Self::cancel_rhai)
            .with_fn("is_cancelled", Self::is_cancelled_rhai);
    }

    pub fn proceed(&mut self) {
        self.cancelled = false;
    }

    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    fn proceed_rhai(s: &mut Arc<Mutex<CallControl>>) -> Result<(), Box<EvalAltResult>> {
        s.lock().map_err(|e| e.to_string())?.proceed();

        Ok(())
    }

    fn cancel_rhai(s: &mut Arc<Mutex<CallControl>>) -> Result<(), Box<EvalAltResult>> {
        s.lock().map_err(|e| e.to_string())?.cancel();

        Ok(())
    }

    fn is_cancelled_rhai(s: &mut Arc<Mutex<CallControl>>) -> Result<bool, Box<EvalAltResult>> {
        Ok(s.lock().map_err(|e| e.to_string())?.cancelled)
    }
}

struct ThreadGuard {
    prev: bool,
}

struct RetBuf<const SIZE: usize> {
    len: usize,
    ents: [(usize, usize); SIZE],
}

enum DispatcherReturn {
    // Normal return path
    Full(usize),
    // Skip calling the original
    Cancel,
    // Bail if we ran out of return slots
    Bail,
}

impl Drop for ThreadGuard {
    fn drop(&mut self) {
        THREAD_GUARD.with(|v| v.set(self.prev));
    }
}

impl Into<isize> for DispatcherReturn {
    fn into(self) -> isize {
        match self {
            DispatcherReturn::Full(a) => a as isize,
            DispatcherReturn::Cancel => -1,
            DispatcherReturn::Bail => 1,
        }
    }
}

impl ThreadGuard {
    fn new() -> ThreadGuard {
        THREAD_GUARD.with(|v| ThreadGuard {
            prev: v.replace(true),
        })
    }

    fn should_quit(&self) -> bool {
        self.prev
    }
}

impl<const SIZE: usize> RetBuf<SIZE> {
    fn insert(&mut self, key: usize, ret: usize) -> bool {
        if let Some(entry) = self.ents[..self.len].iter_mut().find(|i| i.0 == key) {
            entry.1 = ret;
            return false;
        }

        if self.len == SIZE {
            return true;
        }

        self.ents[self.len] = (key, ret);

        self.len += 1;

        false
    }

    fn take(&mut self, key: usize) -> Option<usize> {
        let i = self.ents[..self.len].iter().rposition(|e| e.0 == key)?;
        let ret = self.ents[i].1;
        self.ents[i] = self.ents[self.len - 1];
        self.len -= 1;

        Some(ret)
    }
}

thread_local! {
    static RETURN_ADDRESSES: RefCell<RetBuf<128>> = RefCell::new(RetBuf { len: 0, ents: [(0, 0); 128] });
    static THREAD_GUARD: Cell<bool> = Cell::new(false);
}

extern "C" fn dispatcher(
    import_id: usize,
    is_prefix: bool,
    register_snapshot: *mut arch::NativeRegisterStruct,
    original_return: usize,
    return_slot_address: usize,
) -> isize {
    let return_value = if is_prefix {
        if let Ok(false) = RETURN_ADDRESSES
            .try_with(|v| v.borrow_mut().insert(return_slot_address, original_return))
        {
            DispatcherReturn::Full(0)
        } else {
            DispatcherReturn::Bail
        }
    } else {
        if let Ok(Some(ret)) =
            RETURN_ADDRESSES.try_with(|v| v.borrow_mut().take(return_slot_address))
        {
            DispatcherReturn::Full(ret)
        } else {
            // We're done for if we don't have the return address saved
            std::process::abort();
        }
    };

    let thread_guard = ThreadGuard::new();

    if thread_guard.should_quit() {
        // If we're here then we're in a reentry loop and should not do anything else
        return return_value.into();
    }

    let reg_snapshot = unsafe { &mut *register_snapshot };

    let stack_base = register_snapshot as usize + size_of_val(reg_snapshot);

    let import_id = ImportId(import_id as u64);

    let hm_guard = match HOOKS.lock() {
        Ok(g) => g,
        Err(e) => {
            warn!("[HOOK DISPATCHER] Hook resolution error: {e}");
            return return_value.into();
        }
    };

    let hook_manager = match hm_guard.get(&import_id) {
        Some(hm) => hm,
        None => {
            warn!("[HOOK DISPATCHER] Illegal hook state!");
            return return_value.into();
        }
    };

    let (mut hooks, mut call_control) = if is_prefix {
        (hook_manager.prefix.clone(), Some(CallControl::default()))
    } else {
        (hook_manager.postfix.clone(), None)
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
                    if let Err(e) = parameter.try_resolve(reg_snapshot, stack_base as u64) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
            Either::Second(map) => {
                for parameter in map.values_mut() {
                    if let Err(e) = parameter.try_resolve(reg_snapshot, stack_base as u64) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
        }

        (ctx.callback)(id, &mut ctx.parameters, call_control.as_mut());

        match &ctx.parameters {
            Either::First(vec) => {
                for parameter in vec.iter() {
                    if let Err(e) = parameter.try_commit(reg_snapshot, stack_base as u64) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
            Either::Second(map) => {
                for parameter in map.values() {
                    if let Err(e) = parameter.try_commit(reg_snapshot, stack_base as u64) {
                        warn!("[HOOK DISPATCHER] Hook {id:?} encountered error: {e}");
                        continue 'hooks;
                    }
                }
            }
        }
    }

    if let Some(call_ctrl) = call_control
        && call_ctrl.is_cancelled()
    {
        DispatcherReturn::Cancel.into()
    } else {
        return_value.into()
    }
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

    let mut import_slot_page = NativeMemoryProvider::get_page(import_slot.ptr)?;

    let slot = unsafe { AtomicUsize::from_ptr(import_slot.ptr as *mut usize) };

    let original_address =
        import_slot_page.with_prot(PageProtections::RW, |_| slot.load(Ordering::Acquire))?;

    // Generating a native trampoline
    let trampoline = generate_trampoline(
        arch::NATIVE_TRAMPOLINE_GEN_DESC,
        import_id,
        original_address as u64,
    );

    let mut trampoline_allocator_lock = TRAMPOLINE_ALLOCATOR.lock()?;

    let (trampoline_page, offset) = trampoline_allocator_lock.fetch_or_alloc()?;

    trampoline_page.with_prot(PageProtections::RW, |ptr| unsafe {
        let trampoline_ptr = ptr.byte_add(offset);

        // Here we write the trampoline into the allocated space
        ptr::copy_nonoverlapping(trampoline.as_ptr(), trampoline_ptr, trampoline.len());
    })?;

    let trampoline_ptr = unsafe { trampoline_page.get_ptr().byte_add(offset) };

    if unsafe {
        !clear_cache::clear_cache(trampoline_ptr, trampoline_ptr.byte_add(trampoline.len()))
    } {
        // Critical error, couldn't clear cache
        return Err(free_trampoline_and_err(
            trampoline_ptr,
            Error::IcacheFlushFailed,
        ));
    }

    // Here we swap the pointer if it hasn't changed in the meantime
    import_slot_page.with_prot(PageProtections::RW, |_| {
        slot.compare_exchange(
            original_address,
            trampoline_ptr as usize,
            Ordering::Release,
            Ordering::Relaxed,
        )
        .map_err(|_| {
            free_trampoline_and_err(trampoline_ptr, Error::SlotChanged(import_slot.clone()))
        })
    })??;

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
    phase: HookPhase,
    status: HookStatus,
    priority: HookOrder,
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

    let list = match phase {
        HookPhase::Prefix => &mut hook_mgr.prefix,
        HookPhase::Postfix => &mut hook_mgr.postfix,
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
        hook_mgr.postfix.remove(index);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retbuf_roundtrip() {
        let mut b = RetBuf {
            len: 0,
            ents: [(0, 0); 128],
        };
        assert!(!b.insert(10, 100));
        assert!(!b.insert(20, 200));
        assert!(!b.insert(10, 111)); // overwrite, no growth
        assert_eq!(b.len, 2);
        assert_eq!(b.take(10), Some(111));
        assert_eq!(b.take(10), None);
        assert_eq!(b.take(20), Some(200));
        assert_eq!(b.len, 0);
        for i in 0..128 {
            assert!(!b.insert(i + 1, i));
        }
        assert!(b.insert(999, 0)); // full
    }
}
