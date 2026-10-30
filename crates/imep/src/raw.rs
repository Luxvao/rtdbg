use imp_core::error::Error;

use crate::safe::Endianness;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ElfHeaderRaw32Bit {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub osabi: u8,
    pub abiversion: u8,
    pub pad: [u8; 7],
    pub e_type: u16,
    pub machine: u16,
    pub e_version: u32,
    pub entry: u32,
    pub phoff: u32,
    pub shoff: u32,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ElfHeaderRaw64Bit {
    pub magic: [u8; 4],
    pub class: u8,
    pub data: u8,
    pub version: u8,
    pub osabi: u8,
    pub abiversion: u8,
    pub pad: [u8; 7],
    pub e_type: u16,
    pub machine: u16,
    pub e_version: u32,
    pub entry: u64,
    pub phoff: u64,
    pub shoff: u64,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ProgramHeaderRaw32Bit {
    pub p_type: u32,
    pub offset: u32,
    pub vaddr: u32,
    pub paddr: u32,
    pub filesz: u32,
    pub memsz: u32,
    pub flags: u32,
    pub align: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ProgramHeaderRaw64Bit {
    pub p_type: u32,
    pub flags: u32,
    pub offset: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub align: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ElfDynRaw32Bit {
    pub tag: i32,
    pub un: u32,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ElfDynRaw64Bit {
    pub tag: i64,
    pub un: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ElfSymRaw32Bit {
    pub name: u32,
    pub value: u32,
    pub size: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct ElfSymRaw64Bit {
    pub name: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub value: u64,
    pub size: u64,
}

#[repr(C, packed)]
pub struct ElfRelRaw32Bit {
    pub offset: u32,
    pub info: u32,
}

#[repr(C, packed)]
pub struct ElfRelRaw64Bit {
    pub offset: u64,
    pub info: u64,
}

#[repr(C, packed)]
pub struct ElfRelaRaw32Bit {
    pub offset: u32,
    pub info: u32,
    pub addend: u32,
}

#[repr(C, packed)]
pub struct ElfRelaRaw64Bit {
    pub offset: u64,
    pub info: u64,
    pub addend: u64,
}

impl ElfHeaderRaw32Bit {
    pub(crate) fn correct_for_endianness(mut self) -> Result<ElfHeaderRaw32Bit, Error> {
        if Endianness::NATIVE == Endianness::try_from(self.data)? {
            return Ok(self);
        }

        self.e_type = self.e_type.swap_bytes();
        self.machine = self.machine.swap_bytes();
        self.e_version = self.e_version.swap_bytes();
        self.entry = self.entry.swap_bytes();
        self.phoff = self.phoff.swap_bytes();
        self.shoff = self.shoff.swap_bytes();
        self.flags = self.flags.swap_bytes();
        self.ehsize = self.ehsize.swap_bytes();
        self.phentsize = self.phentsize.swap_bytes();
        self.phnum = self.phnum.swap_bytes();
        self.shentsize = self.shentsize.swap_bytes();
        self.shnum = self.shnum.swap_bytes();
        self.shstrndx = self.shstrndx.swap_bytes();

        Ok(self)
    }
}

impl ElfHeaderRaw64Bit {
    pub(crate) fn correct_for_endianness(mut self) -> Result<ElfHeaderRaw64Bit, Error> {
        if Endianness::NATIVE == Endianness::try_from(self.data)? {
            return Ok(self);
        }

        self.e_type = self.e_type.swap_bytes();
        self.machine = self.machine.swap_bytes();
        self.e_version = self.e_version.swap_bytes();
        self.entry = self.entry.swap_bytes();
        self.phoff = self.phoff.swap_bytes();
        self.shoff = self.shoff.swap_bytes();
        self.flags = self.flags.swap_bytes();
        self.ehsize = self.ehsize.swap_bytes();
        self.phentsize = self.phentsize.swap_bytes();
        self.phnum = self.phnum.swap_bytes();
        self.shentsize = self.shentsize.swap_bytes();
        self.shnum = self.shnum.swap_bytes();
        self.shstrndx = self.shstrndx.swap_bytes();

        Ok(self)
    }
}

impl ProgramHeaderRaw32Bit {
    pub(crate) fn correct_for_endianness(
        mut self,
        endianness: Endianness,
    ) -> ProgramHeaderRaw32Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.p_type = self.p_type.swap_bytes();
        self.offset = self.offset.swap_bytes();
        self.vaddr = self.vaddr.swap_bytes();
        self.paddr = self.paddr.swap_bytes();
        self.filesz = self.filesz.swap_bytes();
        self.memsz = self.memsz.swap_bytes();
        self.flags = self.flags.swap_bytes();
        self.align = self.align.swap_bytes();

        self
    }
}

impl ProgramHeaderRaw64Bit {
    pub(crate) fn correct_for_endianness(
        mut self,
        endianness: Endianness,
    ) -> ProgramHeaderRaw64Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.p_type = self.p_type.swap_bytes();
        self.offset = self.offset.swap_bytes();
        self.vaddr = self.vaddr.swap_bytes();
        self.paddr = self.paddr.swap_bytes();
        self.filesz = self.filesz.swap_bytes();
        self.memsz = self.memsz.swap_bytes();
        self.flags = self.flags.swap_bytes();
        self.align = self.align.swap_bytes();

        self
    }
}

impl ElfDynRaw32Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfDynRaw32Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.tag = self.tag.swap_bytes();
        self.un = self.un.swap_bytes();

        self
    }
}

impl ElfDynRaw64Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfDynRaw64Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.tag = self.tag.swap_bytes();
        self.un = self.un.swap_bytes();

        self
    }
}

impl ElfSymRaw32Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfSymRaw32Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.name = self.name.swap_bytes();
        self.value = self.value.swap_bytes();
        self.size = self.size.swap_bytes();
        self.shndx = self.shndx.swap_bytes();

        self
    }
}

impl ElfSymRaw64Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfSymRaw64Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.name = self.name.swap_bytes();
        self.shndx = self.shndx.swap_bytes();
        self.value = self.value.swap_bytes();
        self.size = self.size.swap_bytes();

        self
    }
}

impl ElfRelRaw32Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfRelRaw32Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.offset = self.offset.swap_bytes();
        self.info = self.info.swap_bytes();

        self
    }
}

impl ElfRelRaw64Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfRelRaw64Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.offset = self.offset.swap_bytes();
        self.info = self.info.swap_bytes();

        self
    }
}

impl ElfRelaRaw32Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfRelaRaw32Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.offset = self.offset.swap_bytes();
        self.info = self.info.swap_bytes();
        self.addend = self.addend.swap_bytes();

        self
    }
}

impl ElfRelaRaw64Bit {
    pub(crate) fn correct_for_endianness(mut self, endianness: Endianness) -> ElfRelaRaw64Bit {
        if Endianness::NATIVE == endianness {
            return self;
        }

        self.offset = self.offset.swap_bytes();
        self.info = self.info.swap_bytes();
        self.addend = self.addend.swap_bytes();

        self
    }
}
