pub mod process;
pub mod runtime_provider;
pub mod vma;

use imep::rhai_enum_impl::*;
use rhai::{Engine, Module, export_module, exported_module};

use crate::register_submodules;

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod linux_module {
    use std::ffi::c_void;

    use crate::os::linux::{process::LinuxProcess, vma::Vma};
    use imep::ElfHeader;
    use rhai::EvalAltResult;

    #[rhai_fn(return_raw)]
    pub fn get_process_info() -> Result<LinuxProcess, Box<EvalAltResult>> {
        Ok(LinuxProcess::this().map_err(|e| e.to_string())?)
    }

    #[rhai_fn(return_raw)]
    pub fn get_vmas() -> Result<Vec<Vma>, Box<EvalAltResult>> {
        Ok(get_process_info()?.vmas.clone())
    }

    #[rhai_fn(return_raw)]
    pub fn get_elf_header() -> Result<ElfHeader, Box<EvalAltResult>> {
        let proc_info = get_process_info()?;

        let header_address = proc_info
            .vmas
            .iter()
            .find(|vma| vma.path.eq(&Some(proc_info.path.clone())) && vma.offset == 0)
            .ok_or("Unable to find ELF header")?
            .saddy;

        Ok(unsafe {
            ElfHeader::parse_at(header_address).map_err(|_| "Unable to parse ELF header")?
        })
    }

    pub fn mprotect(addy: i64, size: i64, prot: i64) -> i64 {
        let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };

        let offset = addy % page_size;

        let addy_aligned = addy - offset;

        let size_with_adjustment = size + offset;

        unsafe {
            i64::from(libc::mprotect(
                addy_aligned as *mut c_void,
                size_with_adjustment as usize,
                prot as i32,
            ))
        }
    }
}

pub fn linux_module(engine: &mut Engine) {
    let mut linux_module = exported_module!(linux_module);

    let mut elf_module = Module::new();

    register_submodules!(elf_module, {
        Class,
        Endianness,
        OsAbi,
        ElfType,
        Machine
    });

    linux_module.set_sub_module("elf", elf_module);

    linux_module.set_var("PROT_NONE", libc::PROT_NONE as i64);
    linux_module.set_var("PROT_READ", libc::PROT_READ as i64);
    linux_module.set_var("PROT_WRITE", libc::PROT_WRITE as i64);
    linux_module.set_var("PROT_EXEC", libc::PROT_EXEC as i64);
    linux_module.set_var("PROT_GROWSUP", libc::PROT_GROWSUP as i64);
    linux_module.set_var("PROT_GROWSDOWN", libc::PROT_GROWSDOWN as i64);

    engine.register_static_module("linux", linux_module.into());
}
