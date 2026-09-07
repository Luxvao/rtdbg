use std::{os::raw::c_void, sync::LazyLock};

use paste::paste;

use librtdbg::{
    elf_utils::{
        Class, ElfHeader, ElfHeaderRaw32Bit, ElfHeaderRaw64Bit, ElfType, Endianness, Machine,
        OsAbi, ProgramType, class_module, elftype_module, endianness_module, machine_module,
        osabi_module, programtype_module,
    },
    parameter::{
        Parameter, ParameterLocation, ParameterValue, parameterlocation_module,
        parametervalue_module,
    },
    proc_utils::{Permissions, Process, Vma, Vmas},
    register::{Register, RegisterSnapshot, register_module},
    register_enums, register_types,
    script::Store,
};
use rhai::{
    Engine, EvalAltResult, FuncRegistration, def_package, exported_module,
    packages::StandardPackage,
};

pub static RTDBG_PACKAGE: LazyLock<RtdbgPackage> = LazyLock::new(|| RtdbgPackage::new());

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

        // Constants
        module.set_var("PROT_NONE", libc::PROT_NONE);
        module.set_var("PROT_READ", libc::PROT_READ);
        module.set_var("PROT_WRITE", libc::PROT_WRITE);
        module.set_var("PROT_EXEC", libc::PROT_EXEC);
        module.set_var("PROT_GROWSUP", libc::PROT_GROWSUP);
        module.set_var("PROT_GROWSDOWN", libc::PROT_GROWSDOWN);
    } |> |engine| {
        setup_types(engine);
        setup_enums(engine);
    }
}

pub fn setup_types(engine: &mut Engine) {
    register_types!(engine, { Permissions, Vma, Vmas, Process, ElfHeader, Store, RegisterSnapshot, Parameter });
}

pub fn setup_enums(engine: &mut Engine) {
    register_enums!(engine, { Class, Endianness, OsAbi, ElfType, Machine, ProgramType, Register, ParameterValue, ParameterLocation });
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
