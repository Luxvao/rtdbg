use std::ffi::{c_int, c_void};

use imp_core::{ImportSlot, Parser};
use libc::{dl_phdr_info, size_t};

use crate::{
    raw::{ElfDynRaw32Bit, ElfDynRaw64Bit, ProgramHeaderRaw32Bit, ProgramHeaderRaw64Bit},
    safe::{Class, ElfDyn, Endianness, ProgramHeader, ProgramType},
};

pub struct ElfParser {}

impl Parser for ElfParser {
    fn import_slots() -> impl Iterator<Item = imp_core::ImportSlot> {
        let mut slots: Vec<ImportSlot> = Vec::new();

        let slots_ptr = (&mut slots) as *mut Vec<ImportSlot> as *mut c_void;

        unsafe {
            libc::dl_iterate_phdr(Some(phdr_callback), slots_ptr);
        }

        slots.into_iter()
    }
}

extern "C" fn phdr_callback(info: *mut dl_phdr_info, _: size_t, data: *mut c_void) -> c_int {
    let slots = unsafe { &mut *(data as *mut Vec<ImportSlot>) };

    let info = unsafe { &*info };

    let (ph_size, dyn_size) = match Class::NATIVE {
        Class::Bits32 => (
            size_of::<ProgramHeaderRaw32Bit>(),
            size_of::<ElfDynRaw32Bit>(),
        ),
        Class::Bits64 => (
            size_of::<ProgramHeaderRaw64Bit>(),
            size_of::<ElfDynRaw64Bit>(),
        ),
    };

    for i in 0..info.dlpi_phnum {
        let ph_address = info.dlpi_phdr as u64 + i as u64 * ph_size as u64;

        let program_header =
            unsafe { ProgramHeader::parse_at(ph_address, Class::NATIVE, Endianness::NATIVE) };

        if program_header.p_type != ProgramType::PtDynamic {
            continue;
        }

        let dyn_arr_size = program_header.memsz / dyn_size as u64;

        let dyn_arr_base_address = info.dlpi_addr + program_header.vaddr;

        for j in 0..dyn_arr_size {
            // Now we're walking the PT_DYNAMIC array
            let dyn_entry_address = dyn_arr_base_address + j as u64 * dyn_size as u64;

            let dyn_entry =
                unsafe { ElfDyn::parse_at(dyn_entry_address, Class::NATIVE, Endianness::NATIVE) };
        }
    }

    0
}
