use std::ffi::{c_int, c_void};

use libc::{
    _SC_PAGESIZE, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_NONE, PROT_READ, PROT_WRITE, sysconf,
};

use crate::{
    error::Error,
    os::{
        linux::{process::LinuxProcess, vma::Permissions},
        memory::{LocationHint, MemoryPage, MemoryProvider, PageProtections},
    },
};

pub struct LinuxMemoryProvider {}

impl MemoryProvider for LinuxMemoryProvider {
    fn page_size() -> usize {
        unsafe { sysconf(_SC_PAGESIZE) as usize }
    }

    fn get_page(addy: u64) -> Result<MemoryPage, Error> {
        let process = LinuxProcess::this()?;

        let page_offset = addy % Self::page_size() as u64;

        let ptr = (addy - page_offset) as *mut u8;

        for vma in process.vmas {
            if vma.saddy <= addy && addy < vma.eaddy {
                return Ok(MemoryPage {
                    ptr,
                    protections: vma_permissions_to_prot(vma.permissions),
                });
            }
        }

        Err(Error::PageNotMapped)
    }

    fn allocate_page(hint: LocationHint, prot: PageProtections) -> Result<MemoryPage, Error> {
        let hint_address: *mut c_void = match hint {
            LocationHint::None => std::ptr::null_mut(),
            LocationHint::Near { loc, max_offset } => todo!(),
            LocationHint::Exact(addr) => addr as *mut c_void,
        };

        let address = unsafe {
            libc::mmap(
                hint_address,
                Self::page_size(),
                prot_to_int(prot),
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };

        if address as i64 == -1 {
            return Err(Error::UnableToAllocatePage);
        }

        Ok(MemoryPage::new(address as *mut u8, prot))
    }

    fn deallocate_page(page: MemoryPage) -> Result<(), Error> {
        if unsafe { libc::munmap(page.get_ptr() as *mut c_void, Self::page_size()) } == -1 {
            return Err(Error::UnableToDeallocatePage);
        }

        Ok(())
    }

    unsafe fn protect_page(page: &mut MemoryPage, prot: PageProtections) -> Result<(), Error> {
        if unsafe {
            libc::mprotect(
                page.get_ptr() as *mut c_void,
                Self::page_size(),
                prot_to_int(prot),
            )
        } == -1
        {
            return Err(Error::UnableToSetPageProtections);
        }

        page.protections = prot;

        Ok(())
    }

    fn safe_read<T: Copy>(addy: u64) -> Result<T, Error> {
        let pid = std::process::id();

        todo!()
    }

    fn safe_write<T>(addy: u64, value: T) -> Result<(), Error> {
        let pid = std::process::id();

        todo!()
    }
}

fn prot_to_int(prot: PageProtections) -> c_int {
    match prot {
        PageProtections::None => PROT_NONE,
        PageProtections::R => PROT_READ,
        PageProtections::W => PROT_WRITE,
        PageProtections::X => PROT_EXEC,
        PageProtections::RW => PROT_READ | PROT_WRITE,
        PageProtections::RX => PROT_READ | PROT_EXEC,
    }
}

fn vma_permissions_to_prot(permissions: Permissions) -> PageProtections {
    if permissions.read && permissions.write {
        return PageProtections::RW;
    }

    if permissions.read && permissions.executable {
        return PageProtections::RX;
    }

    if permissions.read {
        return PageProtections::R;
    }

    if permissions.write {
        return PageProtections::W;
    }

    if permissions.executable {
        return PageProtections::X;
    }

    PageProtections::None
}
