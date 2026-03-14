use rhai::{CustomType, TypeBuilder};

use crate::error::Error;

macro_rules! gen_enum_match {
    ($value:expr, $type:ty, $base:ident, { $( $case:ident ),+ }) => {
        'found: {
            $(
                if $value == $base::$case as $type {
                    break 'found Some($base::$case);
                }
            )+
            None
        }
    };
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    Bits32,
    Bits64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Endianness {
    Little,
    Big,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OsAbi {
    SystemV = 0x00,
    HpUx = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    GNUHurd = 0x04,
    Solaris = 0x06,
    AIX = 0x07,
    IRIX = 0x08,
    FreeBSD = 0x09,
    Tru64 = 0x0A,
    NovellModesto = 0x0B,
    OpenBSD = 0x0C,
    OpenVMS = 0x0D,
    NonStopKernel = 0x0E,
    AROS = 0x0F,
    FenixOS = 0x10,
    NuxiCloudABI = 0x11,
    StratusTechnologiesOpenVOS = 0x12,
}

#[derive(Clone, Copy, Debug)]
pub enum Type {
    EtNone = 0x00,
    EtRel = 0x01,
    EtExec = 0x02,
    EtDyn = 0x03,
    EtCore = 0x04,
    // OS specific
    EtLoos = 0xFE00,
    EtHios = 0xFEFF,
    // Processor specific
    EtLoproc = 0xFF00,
    EtHiproc = 0xFFFF,
}

// Completely unnecessary by the way. I just had time
#[derive(Clone, Copy, Debug)]
pub enum Machine {
    None = 0x00,
    AtNtWe32100 = 0x01,
    Sparc = 0x02,
    X86 = 0x03,
    Motorola68k = 0x04,
    Motorola88k = 0x05,
    IntelMcu = 0x06,
    Intel80860 = 0x07,
    Mips = 0x08,
    IbmSystem370 = 0x09,
    MipsRs3000LE = 0x0A,
    HpPaRisc = 0x0F,
    Intel80960 = 0x13,
    PowerPc = 0x14,
    PowerPc64 = 0x15,
    S390x = 0x16,
    IbmSpc = 0x17,
    NecV800 = 0x24,
    FujistuFr20 = 0x25,
    TrwRh32 = 0x26,
    MotorolaRce = 0x27,
    AArch32 = 0x28,
    DigitalAlpha = 0x29,
    SuperH = 0x2A,
    SparcV9 = 0x2B,
    SiemensTriCoreEP = 0x2C,
    ArgonautRiscCore = 0x2D,
    Hitachi300 = 0x2E,
    Hitachi300H = 0x2F,
    HitachiH8S = 0x30,
    Hitachi500 = 0x31,
    Ia64 = 0x32,
    StanfordMipsX = 0x33,
    MotorolaColdFire = 0x34,
    MotorolaM68HC12 = 0x35,
    FujitsuMmaMultimediaAccelerator = 0x36,
    SiemensPcp = 0x37,
    SonyNCpuERiscP = 0x38,
    DensoNdr1Mcp = 0x39,
    MotorolaStarCoreP = 0x3A,
    ToyotaMe16P = 0x3B,
    STMicroelectronicsSt100P = 0x3C,
    TinyJ = 0x3D,
    X86_64 = 0x3E,
    SonyDspP = 0x3F,
    DigitalEquipmentCorpPdp10 = 0x40,
    DigitalEquipmentCorpPdp11 = 0x41,
    SiemensFx66Mcu = 0x42,
    STMicroelectronicsSt9 = 0x43,
    StMicroelectronicsSt7 = 0x44,
    MotorolaMC68HC16Mcu = 0x45,
    MotorolaMC68HC11Mcu = 0x46,
    MotorolaMC68HC08 = 0x47,
    MotorolaMC68HC05Mcu = 0x48,
    SiliconGraphicsSvX = 0x49,
    STMicroelectronicsSt19 = 0x4A,
    DigitalVax = 0x4B,
    AxisCommuncationsMcp = 0x4C,
    InfineonTechnologiesMcp = 0x4D,
    Element14DspP = 0x4E,
    LsiLogicDspP = 0x4F,
    TMS320C6000 = 0x8C,
    McstElbrusE2K = 0xAF,
    AArch64 = 0xB7,
    ZilogZ80 = 0xDC,
    RiscV = 0xF3,
    BerkeleyPacketFilter = 0xF7,
    Wdc65C816 = 0x101,
    LoongArch = 0x102,
}

#[derive(Clone, Debug, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct ElfHeader {
    #[rhai_type(readonly)]
    pub class: Class,
    #[rhai_type(readonly)]
    pub data: Endianness,
    #[rhai_type(skip)]
    pub version: u8,
    #[rhai_type(readonly)]
    pub osabi: OsAbi,
    #[rhai_type(skip)]
    pub abiversion: u8,
    #[rhai_type(readonly)]
    pub e_type: Type,
    #[rhai_type(readonly)]
    pub machine: Machine,
    #[rhai_type(skip)]
    pub e_version: u32,
    #[rhai_type(skip)]
    pub entry: u64,
    #[rhai_type(skip)]
    pub phoff: u64,
    #[rhai_type(skip)]
    pub shoff: u64,
    #[rhai_type(skip)]
    pub flags: u32,
    #[rhai_type(skip)]
    pub ehsize: u16,
    #[rhai_type(skip)]
    pub phentsize: u16,
    #[rhai_type(skip)]
    pub phnum: u16,
    #[rhai_type(skip)]
    pub shentsize: u16,
    #[rhai_type(skip)]
    pub shnum: u16,
    #[rhai_type(skip)]
    pub shstrndx: u16,
}

impl TryFrom<u8> for Class {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Class::Bits32),
            2 => Ok(Class::Bits64),
            _ => Err(Error::ElfHeaderParsingError),
        }
    }
}

impl TryFrom<u8> for Endianness {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Endianness::Little),
            2 => Ok(Endianness::Big),
            _ => Err(Error::ElfHeaderParsingError),
        }
    }
}

impl TryFrom<u8> for OsAbi {
    type Error = crate::error::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        gen_enum_match!(value, u8, OsAbi, { SystemV, HpUx, NetBSD, Linux, GNUHurd, Solaris, AIX, IRIX, FreeBSD, Tru64, NovellModesto, OpenBSD, OpenVMS, NonStopKernel, AROS, FenixOS, NuxiCloudABI, StratusTechnologiesOpenVOS }).ok_or(Error::ElfHeaderParsingError)
    }
}

impl TryFrom<u16> for Type {
    type Error = crate::error::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        gen_enum_match!(value, u16, Type, {EtNone, EtRel, EtExec, EtDyn, EtCore, EtLoos, EtHios, EtLoproc, EtHiproc}).ok_or(Error::ElfHeaderParsingError)
    }
}

impl TryFrom<u16> for Machine {
    type Error = crate::error::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        gen_enum_match!(value, u16, Machine, {None, AtNtWe32100, Sparc, X86, Motorola68k, Motorola88k, IntelMcu, Intel80860, Mips, IbmSystem370, MipsRs3000LE, HpPaRisc, Intel80960, PowerPc, PowerPc64, S390x, IbmSpc, NecV800, FujistuFr20, TrwRh32, MotorolaRce, AArch32, DigitalAlpha, SuperH, SparcV9, SiemensTriCoreEP, ArgonautRiscCore, Hitachi300, Hitachi300H, HitachiH8S, Hitachi500, Ia64, StanfordMipsX, MotorolaColdFire, MotorolaM68HC12, FujitsuMmaMultimediaAccelerator, SiemensPcp, SonyNCpuERiscP, DensoNdr1Mcp, MotorolaStarCoreP, ToyotaMe16P, STMicroelectronicsSt100P, TinyJ, X86_64, SonyDspP, DigitalEquipmentCorpPdp10, DigitalEquipmentCorpPdp11, SiemensFx66Mcu, STMicroelectronicsSt9, StMicroelectronicsSt7, MotorolaMC68HC16Mcu, MotorolaMC68HC11Mcu, MotorolaMC68HC08, MotorolaMC68HC05Mcu, SiliconGraphicsSvX, STMicroelectronicsSt19, DigitalVax, AxisCommuncationsMcp, InfineonTechnologiesMcp, Element14DspP, LsiLogicDspP, TMS320C6000, McstElbrusE2K, AArch64, ZilogZ80, RiscV, BerkeleyPacketFilter, Wdc65C816, LoongArch}).ok_or(Error::ElfHeaderParsingError)
    }
}

impl TryFrom<ElfHeaderRaw32Bit> for ElfHeader {
    type Error = crate::error::Error;

    fn try_from(mut value: ElfHeaderRaw32Bit) -> Result<Self, Self::Error> {
        if value.magic != [0x7F, b'E', b'L', b'F'] {
            return Err(Error::ElfHeaderParsingError);
        }

        value = value.correct_for_endianness()?;

        Ok(ElfHeader {
            class: Class::try_from(value.class)?,
            data: Endianness::try_from(value.data)?,
            version: value.version,
            osabi: OsAbi::try_from(value.osabi)?,
            abiversion: value.abiversion,
            e_type: Type::try_from(value.e_type)?,
            machine: Machine::try_from(value.machine)?,
            e_version: value.e_version,
            entry: value.entry as u64,
            phoff: value.phoff as u64,
            shoff: value.shoff as u64,
            flags: value.flags,
            ehsize: value.ehsize,
            phentsize: value.phentsize,
            phnum: value.phnum,
            shentsize: value.shentsize,
            shnum: value.shnum,
            shstrndx: value.shstrndx,
        })
    }
}

impl TryFrom<ElfHeaderRaw64Bit> for ElfHeader {
    type Error = crate::error::Error;

    fn try_from(mut value: ElfHeaderRaw64Bit) -> Result<Self, Self::Error> {
        if value.magic != [0x7F, b'E', b'L', b'F'] {
            return Err(Error::ElfHeaderParsingError);
        }

        value = value.correct_for_endianness()?;

        Ok(ElfHeader {
            class: Class::try_from(value.class)?,
            data: Endianness::try_from(value.data)?,
            version: value.version,
            osabi: OsAbi::try_from(value.osabi)?,
            abiversion: value.abiversion,
            e_type: Type::try_from(value.e_type)?,
            machine: Machine::try_from(value.machine)?,
            e_version: value.e_version,
            entry: value.entry,
            phoff: value.phoff,
            shoff: value.shoff,
            flags: value.flags,
            ehsize: value.ehsize,
            phentsize: value.phentsize,
            phnum: value.phnum,
            shentsize: value.shentsize,
            shnum: value.shnum,
            shstrndx: value.shstrndx,
        })
    }
}

impl ElfHeaderRaw32Bit {
    pub fn correct_for_endianness(mut self) -> Result<ElfHeaderRaw32Bit, Error> {
        #[cfg(target_endian = "little")]
        let target_endianness = Endianness::Little;

        #[cfg(target_endian = "big")]
        let target_endianness = Endianness::Big;

        if target_endianness == Endianness::try_from(self.data)? {
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
    pub fn correct_for_endianness(mut self) -> Result<ElfHeaderRaw64Bit, Error> {
        let target_endianness = {
            #[cfg(target_endian = "little")]
            {
                Endianness::Little
            }

            #[cfg(target_endian = "big")]
            {
                Endianness::Big
            }
        };

        if target_endianness == Endianness::try_from(self.data)? {
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

impl ElfHeader {
    fn get_version(&mut self) -> i64 {
        self.version as i64
    }

    fn get_abiversion(&mut self) -> i64 {
        self.abiversion as i64
    }

    fn get_e_version(&mut self) -> i64 {
        self.e_version as i64
    }

    fn get_entry(&mut self) -> i64 {
        self.entry as i64
    }

    fn get_phoff(&mut self) -> i64 {
        self.phoff as i64
    }

    fn get_shoff(&mut self) -> i64 {
        self.shoff as i64
    }

    fn get_flags(&mut self) -> i64 {
        self.flags as i64
    }

    fn get_ehsize(&mut self) -> i64 {
        self.ehsize as i64
    }

    fn get_phentsize(&mut self) -> i64 {
        self.phentsize as i64
    }

    fn get_phnum(&mut self) -> i64 {
        self.phnum as i64
    }

    fn get_shentsize(&mut self) -> i64 {
        self.shentsize as i64
    }

    fn get_shnum(&mut self) -> i64 {
        self.shnum as i64
    }

    fn get_shstrndx(&mut self) -> i64 {
        self.shstrndx as i64
    }

    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder
            .with_get("version", Self::get_version)
            .with_get("abiversion", Self::get_abiversion)
            .with_get("e_version", Self::get_e_version)
            .with_get("entry", Self::get_entry)
            .with_get("phoff", Self::get_phoff)
            .with_get("shoff", Self::get_shoff)
            .with_get("flags", Self::get_flags)
            .with_get("ehsize", Self::get_ehsize)
            .with_get("phentsize", Self::get_phentsize)
            .with_get("phnum", Self::get_phnum)
            .with_get("shentsize", Self::get_shentsize)
            .with_get("shnum", Self::get_shnum)
            .with_get("shstrndx", Self::get_shstrndx)
            .on_print(|header| format!("{header:?}"));
    }
}
