use std::ffi::{CStr, c_int, c_void};

use imp_core::{ImportSlot, Parser};
use libc::{dl_phdr_info, size_t};

use crate::{
    raw::{
        ElfDynRaw32Bit, ElfDynRaw64Bit, ElfRelRaw32Bit, ElfRelRaw64Bit, ElfRelaRaw32Bit,
        ElfRelaRaw64Bit, ElfSymRaw32Bit, ElfSymRaw64Bit, ProgramHeaderRaw32Bit,
        ProgramHeaderRaw64Bit,
    },
    safe::{
        Class, ElfDyn, ElfRel, ElfRela, ElfSym, Endianness, Machine, ProgramHeader, ProgramType,
        RelocType,
    },
};

pub struct ElfParser {}

#[derive(Clone, Copy, Debug)]
pub struct ParseInfo {
    libc: Libc,
    class: Class,
    endianness: Endianness,
    machine: Machine,
    first_run: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Libc {
    Glibc,
    Other,
}

impl Parser for ElfParser {
    fn import_slots<ParseInfo>(extra: ParseInfo) -> impl Iterator<Item = imp_core::ImportSlot> {
        let mut data: (Vec<ImportSlot>, ParseInfo) = (Vec::new(), extra);

        unsafe {
            libc::dl_iterate_phdr(
                Some(phdr_callback),
                (&mut data) as *mut (Vec<ImportSlot>, ParseInfo) as *mut c_void,
            );
        }

        data.0.into_iter()
    }
}

impl ParseInfo {
    pub fn new(libc: Libc, class: Class, endianness: Endianness, machine: Machine) -> ParseInfo {
        ParseInfo {
            libc,
            class,
            endianness,
            machine,
            first_run: true,
        }
    }
}

impl Libc {
    pub const COMPILED_WITH: Libc = if cfg!(target_env = "gnu") {
        Libc::Glibc
    } else {
        Libc::Other
    };
}

extern "C" fn phdr_callback(info: *mut dl_phdr_info, _: size_t, data: *mut c_void) -> c_int {
    let (slots, parse_info) = unsafe { &mut *(data as *mut (Vec<ImportSlot>, ParseInfo)) };

    let info = unsafe { &*info };

    // Get module name
    let module_name = if parse_info.first_run {
        // On both glibc and musl it's the first entry
        parse_info.first_run = false;
        "<main>".to_string()
    } else {
        unsafe {
            CStr::from_ptr(info.dlpi_name)
                .to_string_lossy()
                .into_owned()
        }
    };

    // glibc patches _DYNAMIC already so we don't have to add anything
    let dynamic_bias = if let Libc::Glibc = parse_info.libc {
        0
    } else {
        info.dlpi_addr
    };

    let (ph_size, dyn_size, rel_size, rela_size, sym_size) = match parse_info.class {
        Class::Bits32 => (
            size_of::<ProgramHeaderRaw32Bit>(),
            size_of::<ElfDynRaw32Bit>(),
            size_of::<ElfRelRaw32Bit>(),
            size_of::<ElfRelaRaw32Bit>(),
            size_of::<ElfSymRaw32Bit>(),
        ),
        Class::Bits64 => (
            size_of::<ProgramHeaderRaw64Bit>(),
            size_of::<ElfDynRaw64Bit>(),
            size_of::<ElfRelRaw64Bit>(),
            size_of::<ElfRelaRaw64Bit>(),
            size_of::<ElfSymRaw64Bit>(),
        ),
    };

    for i in 0..info.dlpi_phnum {
        let ph_address = info.dlpi_phdr as u64 + i as u64 * ph_size as u64;

        let program_header =
            unsafe { ProgramHeader::parse_at(ph_address, parse_info.class, parse_info.endianness) };

        if program_header.p_type != ProgramType::PtDynamic {
            continue;
        }

        let dyn_arr_size = program_header.memsz / dyn_size as u64;

        let dyn_arr_base_address = info.dlpi_addr + program_header.vaddr;

        // Entires we need
        let mut symtab = None;
        let mut strtab = None;

        // PLT specific
        let mut pltrel = None;
        let mut pltrelsz = None;
        let mut jmprel = None;

        // Global
        let mut rel = None;
        let mut relsz = None;
        let mut rela = None;
        let mut relasz = None;

        for j in 0..dyn_arr_size {
            // Now we're walking the PT_DYNAMIC array
            let dyn_entry_address = dyn_arr_base_address + j as u64 * dyn_size as u64;

            match unsafe {
                ElfDyn::parse_at(dyn_entry_address, parse_info.class, parse_info.endianness)
            } {
                ElfDyn::Null => break,

                ElfDyn::SymTab(val) => symtab = Some(val + dynamic_bias),
                ElfDyn::StrTab(val) => strtab = Some(val + dynamic_bias),

                ElfDyn::PltRel(val) => pltrel = Some(val),
                ElfDyn::PltRelSz(val) => pltrelsz = Some(val),
                ElfDyn::JmpRel(val) => jmprel = Some(val + dynamic_bias),

                ElfDyn::Rel(val) => rel = Some(val + dynamic_bias),
                ElfDyn::RelSz(val) => relsz = Some(val),
                ElfDyn::Rela(val) => rela = Some(val + dynamic_bias),
                ElfDyn::RelaSz(val) => relasz = Some(val),

                _ => continue,
            }
        }

        // PLT/GOT part
        if let (Some(symtab), Some(strtab), Some(pltrel), Some(pltrelsz), Some(jmprel)) =
            (symtab, strtab, pltrel, pltrelsz, jmprel)
        {
            // If we're here then we've extracted all the _DYNAMIC entries we need to make a map of GOT entries
            let entry_count = pltrelsz
                / if pltrel == 7 {
                    rela_size as u64
                } else {
                    rel_size as u64
                };

            for index in 0..entry_count {
                let (address, symbol_index) = if pltrel == 7 {
                    // Rela
                    let rela_entry = unsafe {
                        ElfRela::parse_at(
                            jmprel + index * rela_size as u64,
                            parse_info.class,
                            parse_info.endianness,
                            parse_info.machine,
                        )
                    };

                    (rela_entry.offset + info.dlpi_addr, rela_entry.symbol_index)
                } else {
                    // Otherwise we just assume Rel
                    let rel_entry = unsafe {
                        ElfRel::parse_at(
                            jmprel + index * rel_size as u64,
                            parse_info.class,
                            parse_info.endianness,
                            parse_info.machine,
                        )
                    };

                    (rel_entry.offset + info.dlpi_addr, rel_entry.symbol_index)
                };

                // Access the symbol entry
                let symbol_entry = unsafe {
                    ElfSym::parse_at(
                        symtab + symbol_index as u64 * sym_size as u64,
                        parse_info.class,
                        parse_info.endianness,
                    )
                };

                // Read the symbol
                let symbol = unsafe {
                    CStr::from_ptr((strtab + symbol_entry.name as u64) as *const i8)
                        .to_string_lossy()
                        .into_owned()
                };

                slots.push(ImportSlot {
                    function: symbol,
                    module: module_name.clone(),
                    ptr: address,
                });
            }
        }

        if let (Some(symtab), Some(strtab), Some(rel), Some(relsz)) = (symtab, strtab, rel, relsz) {
            // Global Rel entries
            let entry_count = relsz / rel_size as u64;

            for index in 0..entry_count {
                let rel_entry = unsafe {
                    ElfRel::parse_at(
                        rel + index * rel_size as u64,
                        parse_info.class,
                        parse_info.endianness,
                        parse_info.machine,
                    )
                };

                if !matches!(rel_entry.r#type, RelocType::GlobDat) {
                    continue;
                }

                let address = rel_entry.offset + info.dlpi_addr;

                let symbol_entry = unsafe {
                    ElfSym::parse_at(
                        symtab + rel_entry.symbol_index as u64 * sym_size as u64,
                        parse_info.class,
                        parse_info.endianness,
                    )
                };

                let symbol = unsafe {
                    CStr::from_ptr((strtab + symbol_entry.name as u64) as *const i8)
                        .to_string_lossy()
                        .into_owned()
                };

                slots.push(ImportSlot {
                    function: symbol,
                    module: module_name.clone(),
                    ptr: address,
                });
            }
        }

        if let (Some(symtab), Some(strtab), Some(rela), Some(relasz)) =
            (symtab, strtab, rela, relasz)
        {
            // Global Rela entries
            let entry_count = relasz / rela_size as u64;

            for index in 0..entry_count {
                let rela_entry = unsafe {
                    ElfRela::parse_at(
                        rela + index * rela_size as u64,
                        parse_info.class,
                        parse_info.endianness,
                        parse_info.machine,
                    )
                };

                if !matches!(rela_entry.r#type, RelocType::GlobDat) {
                    continue;
                }

                let address = rela_entry.offset + info.dlpi_addr;

                let symbol_entry = unsafe {
                    ElfSym::parse_at(
                        symtab + rela_entry.symbol_index as u64,
                        parse_info.class,
                        parse_info.endianness,
                    )
                };

                let symbol = unsafe {
                    CStr::from_ptr((strtab + symbol_entry.name as u64) as *const i8)
                        .to_string_lossy()
                        .into_owned()
                };

                slots.push(ImportSlot {
                    function: symbol,
                    module: module_name.clone(),
                    ptr: address,
                });
            }
        }
    }

    0
}
