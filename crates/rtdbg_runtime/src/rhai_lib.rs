use std::os::raw::c_void;

use librtdbg::{
    elf_utils::{ElfHeader, ElfHeaderRaw32Bit, ElfHeaderRaw64Bit},
    proc_utils::{Permissions, Process, Vma, Vmas},
    register_const, register_fns, register_types,
};
use rhai::{Engine, EvalAltResult, Scope};

pub fn setup_functions(engine: &mut Engine) {
    register_fns!(engine, {
        "get_proc_info" => get_proc_info,
        "get_elf_header" => get_elf_header,
        "get_vmas" => get_vmas,
        "read_mem" => read_mem,
        "write_mem" => write_mem_arr,
        "write_mem" => write_mem_string,
        "mprotect" => mprotect_rhai
    });
}

pub fn setup_constants(scope: &mut Scope) {
    register_const!(scope, {
        "PROT_NONE" => libc::PROT_NONE,
        "PROT_READ" => libc::PROT_READ,
        "PROT_WRITE" => libc::PROT_WRITE,
        "PROT_EXEC" => libc::PROT_EXEC,
        "PROT_GROWSUP" => libc::PROT_GROWSUP,
        "PROT_GROWSDOWN" => libc::PROT_GROWSDOWN
    });
}

pub fn setup_types(engine: &mut Engine) {
    register_types!(engine, { Permissions, Vma, Vmas, Process, ElfHeader });
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
        .filter(|vma| vma.path.eq(&Some(proc_info.path.clone())) && vma.offset == 0)
        .nth(0)
        .ok_or("Unable to find ELF header")?
        .saddy;

    unsafe {
        let elf_header_32 = *(header_address as *const ElfHeaderRaw32Bit);

        if elf_header_32.class == 2 {
            let elf_header_64 = *(header_address as *const ElfHeaderRaw64Bit);

            return ElfHeader::try_from(elf_header_64).map_err(|e| format!("{e}").into());
        }

        ElfHeader::try_from(elf_header_32).map_err(|e| format!("{e}").into())
    }
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
