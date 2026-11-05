use std::marker::PhantomData;

use crate::{error::Error, os::NativeMemoryProvider};

struct ProtGuard<'a, P: MemoryProvider> {
    page: &'a mut MemoryPage,
    old_prot: PageProtections,
    _phantom: PhantomData<P>,
}

impl<'a, P: MemoryProvider> ProtGuard<'a, P> {
    fn new(page: &'a mut MemoryPage, prot: PageProtections) -> Result<ProtGuard<'a, P>, Error> {
        let old_prot = page.protections;

        unsafe {
            P::protect_page(page, prot)?;
        }

        Ok(ProtGuard {
            page,
            old_prot,
            _phantom: PhantomData::default(),
        })
    }
}

impl<'a, P: MemoryProvider> Drop for ProtGuard<'a, P> {
    fn drop(&mut self) {
        unsafe {
            P::protect_page(&mut self.page, self.old_prot)
                .expect("Failed to set protections in the guard!");
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PageProtections {
    None,
    R,
    W,
    X,
    RW,
    RX,
}

#[derive(Debug)]
pub struct MemoryPage {
    pub(crate) ptr: *mut u8,
    pub(crate) protections: PageProtections,
}

// *mut u8 does not implement Send, so we implement it
unsafe impl Send for MemoryPage {}

#[derive(Debug, Copy, Clone)]
pub enum LocationHint {
    None,
    Near { loc: u64, max_offset: u64 },
    Exact(u64),
}

impl MemoryPage {
    pub(crate) fn new(ptr: *mut u8, prot: PageProtections) -> MemoryPage {
        MemoryPage {
            ptr,
            protections: prot,
        }
    }

    pub fn get_ptr(&self) -> *mut u8 {
        self.ptr
    }

    pub fn get_protections(&self) -> PageProtections {
        self.protections
    }

    pub fn with_prot<F, T>(&mut self, prot: PageProtections, f: F) -> Result<T, Error>
    where
        F: FnOnce(*mut u8) -> T,
    {
        self.with_prot_in::<NativeMemoryProvider, _, _>(prot, f)
    }

    pub fn with_prot_in<P, F, T>(&mut self, prot: PageProtections, f: F) -> Result<T, Error>
    where
        P: MemoryProvider,
        F: FnOnce(*mut u8) -> T,
    {
        let ptr = self.ptr;

        let _prot_guard = ProtGuard::<P>::new(self, prot)?;

        Ok(f(ptr))
    }
}

pub trait MemoryProvider {
    fn page_size() -> usize;

    /// Addresses passed in are automatically aligned
    fn get_page(addy: u64) -> Result<MemoryPage, Error>;

    fn allocate_page(hint: LocationHint, prot: PageProtections) -> Result<MemoryPage, Error>;
    fn deallocate_page(page: MemoryPage) -> Result<(), Error>;

    unsafe fn protect_page(page: &mut MemoryPage, prot: PageProtections) -> Result<(), Error>;

    fn safe_read<T: Copy>(addy: u64) -> Result<T, Error>;
    fn safe_write<T>(addy: u64, value: T) -> Result<(), Error>;
}
