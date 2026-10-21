use std::ptr::read_unaligned;

use imp_core::Error;
use rhai::{CustomType, TypeBuilder};

use crate::raw::{
    ElfDynRaw32Bit, ElfDynRaw64Bit, ElfHeaderRaw32Bit, ElfHeaderRaw64Bit, ElfRelRaw32Bit,
    ElfRelRaw64Bit, ElfRelaRaw32Bit, ElfRelaRaw64Bit, ElfSymRaw32Bit, ElfSymRaw64Bit,
    ProgramHeaderRaw32Bit, ProgramHeaderRaw64Bit,
};

macro_rules! gen_enum_match {
    ($value:expr, $base:ident, $default:ident, { $( $case:ident = $val:expr ),+ }) => {
        match $value {
            $($val => $base::$case,)+
            v => $base::$default(v),
        }
    };
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
    SystemV,
    HpUx,
    NetBSD,
    Linux,
    GNUHurd,
    Solaris,
    AIX,
    IRIX,
    FreeBSD,
    Tru64,
    NovellModesto,
    OpenBSD,
    OpenVMS,
    NonStopKernel,
    AROS,
    FenixOS,
    NuxiCloudABI,
    StratusTechnologiesOpenVOS,
    Unknown(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum ElfType {
    EtNone,
    EtRel,
    EtExec,
    EtDyn,
    EtCore,
    // OS specific
    EtLoos,
    EtHios,
    // Processor specific
    EtLoproc,
    EtHiproc,
    Unknown(u16),
}

// Completely unnecessary by the way. I just had time
#[derive(Clone, Copy, Debug)]
pub enum Machine {
    None,
    AtNtWe32100,
    Sparc,
    X86,
    Motorola68k,
    Motorola88k,
    IntelMcu,
    Intel80860,
    Mips,
    IbmSystem370,
    MipsRs3000LE,
    HpPaRisc,
    Intel80960,
    PowerPc,
    PowerPc64,
    S390x,
    IbmSpc,
    NecV800,
    FujistuFr20,
    TrwRh32,
    MotorolaRce,
    AArch32,
    DigitalAlpha,
    SuperH,
    SparcV9,
    SiemensTriCoreEP,
    ArgonautRiscCore,
    Hitachi300,
    Hitachi300H,
    HitachiH8S,
    Hitachi500,
    Ia64,
    StanfordMipsX,
    MotorolaColdFire,
    MotorolaM68HC12,
    FujitsuMmaMultimediaAccelerator,
    SiemensPcp,
    SonyNCpuERiscP,
    DensoNdr1Mcp,
    MotorolaStarCoreP,
    ToyotaMe16P,
    STMicroelectronicsSt100P,
    TinyJ,
    X86_64,
    SonyDspP,
    DigitalEquipmentCorpPdp10,
    DigitalEquipmentCorpPdp11,
    SiemensFx66Mcu,
    STMicroelectronicsSt9,
    StMicroelectronicsSt7,
    MotorolaMC68HC16Mcu,
    MotorolaMC68HC11Mcu,
    MotorolaMC68HC08,
    MotorolaMC68HC05Mcu,
    SiliconGraphicsSvX,
    STMicroelectronicsSt19,
    DigitalVax,
    AxisCommuncationsMcp,
    InfineonTechnologiesMcp,
    Element14DspP,
    LsiLogicDspP,
    TMS320C6000,
    McstElbrusE2K,
    AArch64,
    ZilogZ80,
    RiscV,
    BerkeleyPacketFilter,
    Wdc65C816,
    LoongArch,
    Unknown(u16),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ProgramType {
    PtNull,
    PtLoad,
    PtDynamic,
    PtInterp,
    PtNote,
    PtShlib,
    PtPhdr,
    PtTls,
    PtLoos,
    PtHios,
    PtLoproc,
    PtHiproc,
    Unknown(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct ProgramFlags(u32);

#[derive(Clone, Debug, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct ElfHeader {
    pub class: Class,
    pub data: Endianness,
    pub version: u8,
    pub osabi: OsAbi,
    pub abiversion: u8,
    pub e_type: ElfType,
    pub machine: Machine,
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

#[derive(Clone, Debug, CustomType)]
#[rhai_type(extra = Self::build_extra)]
pub struct ProgramHeader {
    pub p_type: ProgramType,
    pub flags: ProgramFlags,
    pub offset: u64,
    pub vaddr: u64,
    pub paddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub align: u64,
}

#[derive(Clone, Debug)]
pub enum ElfDyn {
    Null,
    Needed(u64),
    PltRelSz(u64),
    PltGot(u64),
    Hash(u64),
    StrTab(u64),
    SymTab(u64),
    Rela(u64),
    RelaSz(u64),
    RelaEnt(u64),
    StrSz(u64),
    SymEnt(u64),
    Init(u64),
    Fini(u64),
    SoName(u64),
    RPath(u64),
    Symbolic,
    Rel(u64),
    RelSz(u64),
    RelEnt(u64),
    PltRel(u64),
    Debug(u64),
    TextRel,
    JmpRel(u64),
    BindNow,
    InitArray(u64),
    FiniArray(u64),
    InitArraySz(u64),
    FiniArraySz(u64),
    RunPath(u64),
    Flags(u64),
    PreInitArray(u64),
    PreInitArraySz(u64),
    SunwAuxiliary(u64),
    SunwRtldInf(u64),
    SunwFilter(u64),
    SunwCap(u64),
    SunwSymTab(u64),
    SunwSymSz(u64),
    SunwSortEnt(u64),
    SunwSymSort(u64),
    SunwSymSortSz(u64),
    SunwTlsSort(u64),
    SunwTlsSortSz(u64),
    SunwCapInfo(u64),
    SunwStrPad(u64),
    SunwCapChain(u64),
    SunwLdMach(u64),
    SunwCapChainEnt(u64),
    SunwCapChainSz(u64),
    Checksum(u64),
    PltPadSz(u64),
    MoveEnt(u64),
    MoveSz(u64),
    PosFlag1(u64),
    SymInSz(u64),
    SymInEnt(u64),
    Config(u64),
    DepAudit(u64),
    Audit(u64),
    PltPad(u64),
    MoveTab(u64),
    SymInfo(u64),
    RelaCount(u64),
    RelCount(u64),
    Flags1(u64),
    VerDef(u64),
    VerDefNum(u64),
    VerNeed(u64),
    VerNeedNum(u64),
    SparcRegister(u64),
    Auxiliary(u64),
    Used(u64),
    Filter(u64),
    Unknown { tag: i64, val: u64 },
}

#[derive(Clone, Copy, Debug, CustomType)]
pub struct ElfSym {
    pub name: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub value: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelocType {
    None,

    Relative,
    Absolute,
    GlobDat,
    JumpSlot,
    Copy,
    IRelative,

    TlsDtpMod,
    TlsDtpOff,
    TlsTpOff,
    TlsDesc,

    PcRel32,
    PcRel64,
    Plt32,
    Got32,
    Got32X,
    GotOff,
    GotPc,
    GotPcRel,
    GotPcRelX,
    RexGotPcRelX,
    Abs32ZeroExt,
    Abs32SignExt,

    TlsGd,
    TlsLd,
    TlsLdo,
    TlsInitialExec,
    TlsLocalExec,

    Call,
    CallPlt,
    Jump24,
    Jump26,
    ThumbCall,
    ThumbJump24,
    Branch,
    Jal,
    RvcBranch,
    RvcJump,

    MovwAbsNc,
    MovtAbs,
    ThumbMovwAbsNc,
    ThumbMovtAbs,
    Prel31,
    Target1,
    Target2,

    AdrPrelLo21,
    AdrPage21,
    AddAbsLo12,
    Ldst8Lo12,
    Ldst16Lo12,
    Ldst32Lo12,
    Ldst64Lo12,
    Ldst128Lo12,
    Tstbr14,
    Condbr19,
    AdrGotPage,
    Ld64GotLo12,
    TlsGdPage21,
    TlsGdLo12,
    TlsLdPage21,
    TlsLdLo12,
    TlsLdDtprelHi12,
    TlsLdDtprelLo12,
    TlsIePage21,
    TlsIeLo12,
    TlsLeTprelHi12,
    TlsLeTprelLo12,
    TlsDescPage21,
    TlsDescLd64Lo12,
    TlsDescAddLo12,
    TlsDescCall,

    GotHi20,
    PcrelHi20,
    PcrelLo12I,
    PcrelLo12S,
    Hi20,
    Lo12I,
    Lo12S,
    TlsGotHi20,
    TlsGdHi20,
    TprelHi20,
    TprelLo12I,
    TprelLo12S,
    TprelAdd,
    Relax,
    Align,
    Add8,
    Add16,
    Add32,
    Add64,
    Sub8,
    Sub16,
    Sub32,
    Sub64,

    Unknown(u32),
}

#[derive(Clone, Copy, Debug, CustomType)]
pub struct ElfRel {
    pub offset: u64,
    pub symbol_index: u32,
    pub r#type: RelocType,
}

#[derive(Clone, Copy, Debug, CustomType)]
pub struct ElfRela {
    pub offset: u64,
    pub symbol_index: u32,
    pub r#type: RelocType,
    pub addend: u64,
}

impl TryFrom<u8> for Class {
    type Error = imp_core::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Class::Bits32),
            2 => Ok(Class::Bits64),
            _ => Err(Error::ElfHeaderParsingError),
        }
    }
}

impl TryFrom<u8> for Endianness {
    type Error = imp_core::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Endianness::Little),
            2 => Ok(Endianness::Big),
            _ => Err(Error::ElfHeaderParsingError),
        }
    }
}

impl From<u8> for OsAbi {
    fn from(value: u8) -> Self {
        gen_enum_match!(value, OsAbi, Unknown, {SystemV = 0x00, HpUx = 0x01, NetBSD = 0x02, Linux = 0x03, GNUHurd = 0x04, Solaris = 0x06, AIX = 0x07, IRIX = 0x08, FreeBSD = 0x09, Tru64 = 0x0A, NovellModesto = 0x0B, OpenBSD = 0x0C, OpenVMS = 0x0D, NonStopKernel = 0x0E, AROS = 0x0F, FenixOS = 0x10, NuxiCloudABI = 0x11, StratusTechnologiesOpenVOS = 0x12})
    }
}

impl From<u16> for ElfType {
    fn from(value: u16) -> Self {
        gen_enum_match!(value, ElfType, Unknown, {EtNone = 0x00, EtRel = 0x01, EtExec = 0x02, EtDyn = 0x03, EtCore = 0x04, EtLoos = 0xFE00, EtHios = 0xFEFF, EtLoproc = 0xFF00, EtHiproc = 0xFFFF})
    }
}

impl From<u16> for Machine {
    fn from(value: u16) -> Self {
        gen_enum_match!(value, Machine, Unknown, {None = 0x00, AtNtWe32100 = 0x01, Sparc = 0x02, X86 = 0x03, Motorola68k = 0x04, Motorola88k = 0x05, IntelMcu = 0x06, Intel80860 = 0x07, Mips = 0x08, IbmSystem370 = 0x09, MipsRs3000LE = 0x0A, HpPaRisc = 0x0F, Intel80960 = 0x13, PowerPc = 0x14, PowerPc64 = 0x15, S390x = 0x16, IbmSpc = 0x17, NecV800 = 0x24, FujistuFr20 = 0x25, TrwRh32 = 0x26, MotorolaRce = 0x27, AArch32 = 0x28, DigitalAlpha = 0x29, SuperH = 0x2A, SparcV9 = 0x2B, SiemensTriCoreEP = 0x2C, ArgonautRiscCore = 0x2D, Hitachi300 = 0x2E, Hitachi300H = 0x2F, HitachiH8S = 0x30, Hitachi500 = 0x31, Ia64 = 0x32, StanfordMipsX = 0x33, MotorolaColdFire = 0x34, MotorolaM68HC12 = 0x35, FujitsuMmaMultimediaAccelerator = 0x36, SiemensPcp = 0x37, SonyNCpuERiscP = 0x38, DensoNdr1Mcp = 0x39, MotorolaStarCoreP = 0x3A, ToyotaMe16P = 0x3B, STMicroelectronicsSt100P = 0x3C, TinyJ = 0x3D, X86_64 = 0x3E, SonyDspP = 0x3F, DigitalEquipmentCorpPdp10 = 0x40, DigitalEquipmentCorpPdp11 = 0x41, SiemensFx66Mcu = 0x42, STMicroelectronicsSt9 = 0x43, StMicroelectronicsSt7 = 0x44, MotorolaMC68HC16Mcu = 0x45, MotorolaMC68HC11Mcu = 0x46, MotorolaMC68HC08 = 0x47, MotorolaMC68HC05Mcu = 0x48, SiliconGraphicsSvX = 0x49, STMicroelectronicsSt19 = 0x4A, DigitalVax = 0x4B, AxisCommuncationsMcp = 0x4C, InfineonTechnologiesMcp = 0x4D, Element14DspP = 0x4E, LsiLogicDspP = 0x4F, TMS320C6000 = 0x8C, McstElbrusE2K = 0xAF, AArch64 = 0xB7, ZilogZ80 = 0xDC, RiscV = 0xF3, BerkeleyPacketFilter = 0xF7, Wdc65C816 = 0x101, LoongArch = 0x102})
    }
}

impl From<u32> for ProgramType {
    fn from(value: u32) -> Self {
        gen_enum_match!(value, ProgramType, Unknown, {PtNull = 0x0, PtLoad = 0x1, PtDynamic = 0x2, PtInterp = 0x3, PtNote = 0x4, PtShlib = 0x5, PtPhdr = 0x6, PtTls = 0x7, PtLoos = 0x60000000, PtHios = 0x6FFFFFFF, PtLoproc = 0x70000000, PtHiproc = 0x7FFFFFFF})
    }
}

impl From<u32> for ProgramFlags {
    fn from(value: u32) -> Self {
        ProgramFlags(value)
    }
}

impl TryFrom<ElfHeaderRaw32Bit> for ElfHeader {
    type Error = imp_core::Error;

    fn try_from(mut value: ElfHeaderRaw32Bit) -> Result<Self, Self::Error> {
        if value.magic != [0x7F, b'E', b'L', b'F'] {
            return Err(Error::ElfHeaderParsingError);
        }

        value = value.correct_for_endianness()?;

        Ok(ElfHeader {
            class: Class::try_from(value.class)?,
            data: Endianness::try_from(value.data)?,
            version: value.version,
            osabi: OsAbi::from(value.osabi),
            abiversion: value.abiversion,
            e_type: ElfType::from(value.e_type),
            machine: Machine::from(value.machine),
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
    type Error = imp_core::Error;

    fn try_from(mut value: ElfHeaderRaw64Bit) -> Result<Self, Self::Error> {
        if value.magic != [0x7F, b'E', b'L', b'F'] {
            return Err(Error::ElfHeaderParsingError);
        }

        value = value.correct_for_endianness()?;

        Ok(ElfHeader {
            class: Class::try_from(value.class)?,
            data: Endianness::try_from(value.data)?,
            version: value.version,
            osabi: OsAbi::from(value.osabi),
            abiversion: value.abiversion,
            e_type: ElfType::from(value.e_type),
            machine: Machine::from(value.machine),
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

impl Class {
    pub const NATIVE: Class = if cfg!(target_pointer_width = "64") {
        Class::Bits64
    } else {
        Class::Bits32
    };
}

impl Endianness {
    pub const NATIVE: Endianness = if cfg!(target_endian = "little") {
        Endianness::Little
    } else {
        Endianness::Big
    };
}

impl ProgramFlags {
    pub const X: u32 = 0x1;
    pub const W: u32 = 0x2;
    pub const R: u32 = 0x4;

    pub fn is_executable(&self) -> bool {
        self.0 & Self::X != 0
    }
    pub fn is_writable(&self) -> bool {
        self.0 & Self::W != 0
    }
    pub fn is_readable(&self) -> bool {
        self.0 & Self::R != 0
    }
}

impl ElfHeader {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.on_print(|header| format!("{header:?}"));
    }

    /// # Safety
    /// `address` must be readable for `size_of(ElfHeaderRaw32/64Bit)` bytes.
    pub unsafe fn parse_at(address: u64) -> Result<ElfHeader, Error> {
        let elf32bit = unsafe { read_unaligned(address as *const ElfHeaderRaw32Bit) };

        // If we're here then we need to read the 64 bit elf header instead
        if elf32bit.class == 2 {
            let elf64bit = unsafe { read_unaligned(address as *const ElfHeaderRaw64Bit) };

            return ElfHeader::try_from(elf64bit);
        }

        ElfHeader::try_from(elf32bit)
    }
}

impl ProgramHeader {
    fn build_extra(builder: &mut TypeBuilder<Self>) {
        builder.on_print(|header| format!("{header:?}"));
    }

    /// # Safety
    /// `address` must be readable for `size_of(ProgramHeaderRaw32/64Bit)` bytes.
    pub unsafe fn parse_at(address: u64, class: Class, endianness: Endianness) -> ProgramHeader {
        match class {
            Class::Bits32 => ProgramHeader::from_32bit(
                unsafe { read_unaligned(address as *const ProgramHeaderRaw32Bit) },
                endianness,
            ),
            Class::Bits64 => ProgramHeader::from_64bit(
                unsafe { read_unaligned(address as *const ProgramHeaderRaw64Bit) },
                endianness,
            ),
        }
    }

    fn from_32bit(mut value: ProgramHeaderRaw32Bit, endianness: Endianness) -> ProgramHeader {
        value = value.correct_for_endianness(endianness);

        ProgramHeader {
            p_type: ProgramType::from(value.p_type),
            flags: ProgramFlags::from(value.flags),
            offset: value.offset as u64,
            vaddr: value.vaddr as u64,
            paddr: value.paddr as u64,
            filesz: value.filesz as u64,
            memsz: value.memsz as u64,
            align: value.align as u64,
        }
    }

    fn from_64bit(mut value: ProgramHeaderRaw64Bit, endianness: Endianness) -> ProgramHeader {
        value = value.correct_for_endianness(endianness);

        ProgramHeader {
            p_type: ProgramType::from(value.p_type),
            flags: ProgramFlags::from(value.flags),
            offset: value.offset,
            vaddr: value.vaddr,
            paddr: value.paddr,
            filesz: value.filesz,
            memsz: value.memsz,
            align: value.align,
        }
    }
}

impl ElfDyn {
    fn from_32bit(mut value: ElfDynRaw32Bit, endianness: Endianness) -> ElfDyn {
        value = value.correct_for_endianness(endianness);

        match value.tag {
            0 => ElfDyn::Null,
            1 => ElfDyn::Needed(value.un as u64),
            2 => ElfDyn::PltRelSz(value.un as u64),
            3 => ElfDyn::PltGot(value.un as u64),
            4 => ElfDyn::Hash(value.un as u64),
            5 => ElfDyn::StrTab(value.un as u64),
            6 => ElfDyn::SymTab(value.un as u64),
            7 => ElfDyn::Rela(value.un as u64),
            8 => ElfDyn::RelaSz(value.un as u64),
            9 => ElfDyn::RelaEnt(value.un as u64),
            10 => ElfDyn::StrSz(value.un as u64),
            11 => ElfDyn::SymEnt(value.un as u64),
            12 => ElfDyn::Init(value.un as u64),
            13 => ElfDyn::Fini(value.un as u64),
            14 => ElfDyn::SoName(value.un as u64),
            15 => ElfDyn::RPath(value.un as u64),
            16 => ElfDyn::Symbolic,
            17 => ElfDyn::Rel(value.un as u64),
            18 => ElfDyn::RelSz(value.un as u64),
            19 => ElfDyn::RelEnt(value.un as u64),
            20 => ElfDyn::PltRel(value.un as u64),
            21 => ElfDyn::Debug(value.un as u64),
            22 => ElfDyn::TextRel,
            23 => ElfDyn::JmpRel(value.un as u64),
            24 => ElfDyn::BindNow,
            25 => ElfDyn::InitArray(value.un as u64),
            26 => ElfDyn::FiniArray(value.un as u64),
            27 => ElfDyn::InitArraySz(value.un as u64),
            28 => ElfDyn::FiniArraySz(value.un as u64),
            29 => ElfDyn::RunPath(value.un as u64),
            30 => ElfDyn::Flags(value.un as u64),
            32 => ElfDyn::PreInitArray(value.un as u64),
            33 => ElfDyn::PreInitArraySz(value.un as u64),

            0x6000000d => ElfDyn::SunwAuxiliary(value.un as u64),
            0x6000000e => ElfDyn::SunwRtldInf(value.un as u64),
            0x6000000f => ElfDyn::SunwFilter(value.un as u64),
            0x60000010 => ElfDyn::SunwCap(value.un as u64),
            0x60000011 => ElfDyn::SunwSymTab(value.un as u64),
            0x60000012 => ElfDyn::SunwSymSz(value.un as u64),
            0x60000013 => ElfDyn::SunwSortEnt(value.un as u64),
            0x60000014 => ElfDyn::SunwSymSort(value.un as u64),
            0x60000015 => ElfDyn::SunwSymSortSz(value.un as u64),
            0x60000016 => ElfDyn::SunwTlsSort(value.un as u64),
            0x60000017 => ElfDyn::SunwTlsSortSz(value.un as u64),
            0x60000018 => ElfDyn::SunwCapInfo(value.un as u64),
            0x60000019 => ElfDyn::SunwStrPad(value.un as u64),
            0x6000001a => ElfDyn::SunwCapChain(value.un as u64),
            0x6000001b => ElfDyn::SunwLdMach(value.un as u64),
            0x6000001d => ElfDyn::SunwCapChainEnt(value.un as u64),
            0x6000001f => ElfDyn::SunwCapChainSz(value.un as u64),

            0x6ffffdf8 => ElfDyn::Checksum(value.un as u64),
            0x6ffffdf9 => ElfDyn::PltPadSz(value.un as u64),
            0x6ffffdfa => ElfDyn::MoveEnt(value.un as u64),
            0x6ffffdfb => ElfDyn::MoveSz(value.un as u64),
            0x6ffffdfd => ElfDyn::PosFlag1(value.un as u64),
            0x6ffffdfe => ElfDyn::SymInSz(value.un as u64),
            0x6ffffdff => ElfDyn::SymInEnt(value.un as u64),
            0x6ffffefa => ElfDyn::Config(value.un as u64),
            0x6ffffefb => ElfDyn::DepAudit(value.un as u64),
            0x6ffffefc => ElfDyn::Audit(value.un as u64),
            0x6ffffefd => ElfDyn::PltPad(value.un as u64),
            0x6ffffefe => ElfDyn::MoveTab(value.un as u64),
            0x6ffffeff => ElfDyn::SymInfo(value.un as u64),
            0x6ffffff9 => ElfDyn::RelaCount(value.un as u64),
            0x6ffffffa => ElfDyn::RelCount(value.un as u64),
            0x6ffffffb => ElfDyn::Flags1(value.un as u64),
            0x6ffffffc => ElfDyn::VerDef(value.un as u64),
            0x6ffffffd => ElfDyn::VerDefNum(value.un as u64),
            0x6ffffffe => ElfDyn::VerNeed(value.un as u64),
            0x6fffffff => ElfDyn::VerNeedNum(value.un as u64),

            0x70000001 => ElfDyn::SparcRegister(value.un as u64),
            0x7ffffffd => ElfDyn::Auxiliary(value.un as u64),
            0x7ffffffe => ElfDyn::Used(value.un as u64),
            0x7fffffff => ElfDyn::Filter(value.un as u64),

            _ => ElfDyn::Unknown {
                tag: value.tag as i64,
                val: value.un as u64,
            },
        }
    }

    fn from_64bit(mut value: ElfDynRaw64Bit, endianness: Endianness) -> ElfDyn {
        value = value.correct_for_endianness(endianness);

        match value.tag {
            0 => ElfDyn::Null,
            1 => ElfDyn::Needed(value.un),
            2 => ElfDyn::PltRelSz(value.un),
            3 => ElfDyn::PltGot(value.un),
            4 => ElfDyn::Hash(value.un),
            5 => ElfDyn::StrTab(value.un),
            6 => ElfDyn::SymTab(value.un),
            7 => ElfDyn::Rela(value.un),
            8 => ElfDyn::RelaSz(value.un),
            9 => ElfDyn::RelaEnt(value.un),
            10 => ElfDyn::StrSz(value.un),
            11 => ElfDyn::SymEnt(value.un),
            12 => ElfDyn::Init(value.un),
            13 => ElfDyn::Fini(value.un),
            14 => ElfDyn::SoName(value.un),
            15 => ElfDyn::RPath(value.un),
            16 => ElfDyn::Symbolic,
            17 => ElfDyn::Rel(value.un),
            18 => ElfDyn::RelSz(value.un),
            19 => ElfDyn::RelEnt(value.un),
            20 => ElfDyn::PltRel(value.un),
            21 => ElfDyn::Debug(value.un),
            22 => ElfDyn::TextRel,
            23 => ElfDyn::JmpRel(value.un),
            24 => ElfDyn::BindNow,
            25 => ElfDyn::InitArray(value.un),
            26 => ElfDyn::FiniArray(value.un),
            27 => ElfDyn::InitArraySz(value.un),
            28 => ElfDyn::FiniArraySz(value.un),
            29 => ElfDyn::RunPath(value.un),
            30 => ElfDyn::Flags(value.un),
            32 => ElfDyn::PreInitArray(value.un),
            33 => ElfDyn::PreInitArraySz(value.un),

            0x6000000d => ElfDyn::SunwAuxiliary(value.un),
            0x6000000e => ElfDyn::SunwRtldInf(value.un),
            0x6000000f => ElfDyn::SunwFilter(value.un),
            0x60000010 => ElfDyn::SunwCap(value.un),
            0x60000011 => ElfDyn::SunwSymTab(value.un),
            0x60000012 => ElfDyn::SunwSymSz(value.un),
            0x60000013 => ElfDyn::SunwSortEnt(value.un),
            0x60000014 => ElfDyn::SunwSymSort(value.un),
            0x60000015 => ElfDyn::SunwSymSortSz(value.un),
            0x60000016 => ElfDyn::SunwTlsSort(value.un),
            0x60000017 => ElfDyn::SunwTlsSortSz(value.un),
            0x60000018 => ElfDyn::SunwCapInfo(value.un),
            0x60000019 => ElfDyn::SunwStrPad(value.un),
            0x6000001a => ElfDyn::SunwCapChain(value.un),
            0x6000001b => ElfDyn::SunwLdMach(value.un),
            0x6000001d => ElfDyn::SunwCapChainEnt(value.un),
            0x6000001f => ElfDyn::SunwCapChainSz(value.un),

            0x6ffffdf8 => ElfDyn::Checksum(value.un),
            0x6ffffdf9 => ElfDyn::PltPadSz(value.un),
            0x6ffffdfa => ElfDyn::MoveEnt(value.un),
            0x6ffffdfb => ElfDyn::MoveSz(value.un),
            0x6ffffdfd => ElfDyn::PosFlag1(value.un),
            0x6ffffdfe => ElfDyn::SymInSz(value.un),
            0x6ffffdff => ElfDyn::SymInEnt(value.un),
            0x6ffffefa => ElfDyn::Config(value.un),
            0x6ffffefb => ElfDyn::DepAudit(value.un),
            0x6ffffefc => ElfDyn::Audit(value.un),
            0x6ffffefd => ElfDyn::PltPad(value.un),
            0x6ffffefe => ElfDyn::MoveTab(value.un),
            0x6ffffeff => ElfDyn::SymInfo(value.un),
            0x6ffffff9 => ElfDyn::RelaCount(value.un),
            0x6ffffffa => ElfDyn::RelCount(value.un),
            0x6ffffffb => ElfDyn::Flags1(value.un),
            0x6ffffffc => ElfDyn::VerDef(value.un),
            0x6ffffffd => ElfDyn::VerDefNum(value.un),
            0x6ffffffe => ElfDyn::VerNeed(value.un),
            0x6fffffff => ElfDyn::VerNeedNum(value.un),

            0x70000001 => ElfDyn::SparcRegister(value.un),
            0x7ffffffd => ElfDyn::Auxiliary(value.un),
            0x7ffffffe => ElfDyn::Used(value.un),
            0x7fffffff => ElfDyn::Filter(value.un),

            _ => ElfDyn::Unknown {
                tag: value.tag,
                val: value.un,
            },
        }
    }

    /// # Safety
    /// `address` must be readable for `size_of(ElfDynRaw32/64Bit)` bytes.
    pub unsafe fn parse_at(address: u64, class: Class, endianness: Endianness) -> ElfDyn {
        match class {
            Class::Bits32 => ElfDyn::from_32bit(
                unsafe { read_unaligned(address as *const ElfDynRaw32Bit) },
                endianness,
            ),
            Class::Bits64 => ElfDyn::from_64bit(
                unsafe { read_unaligned(address as *const ElfDynRaw64Bit) },
                endianness,
            ),
        }
    }
}

impl ElfSym {
    pub fn from_32bit(mut value: ElfSymRaw32Bit, endianness: Endianness) -> ElfSym {
        value = value.correct_for_endianness(endianness);

        ElfSym {
            name: value.name,
            info: value.info,
            other: value.other,
            shndx: value.shndx,
            value: value.value as u64,
            size: value.size as u64,
        }
    }

    pub fn from_64bit(mut value: ElfSymRaw64Bit, endianness: Endianness) -> ElfSym {
        value = value.correct_for_endianness(endianness);

        ElfSym {
            name: value.name,
            info: value.info,
            other: value.other,
            shndx: value.shndx,
            value: value.value,
            size: value.size,
        }
    }

    /// # Safety
    /// `address` must be readable for `size_of(ElfSymRaw32/64Bit)` bytes.
    pub unsafe fn parse_at(address: u64, class: Class, endianness: Endianness) -> ElfSym {
        match class {
            Class::Bits32 => ElfSym::from_32bit(
                unsafe { read_unaligned(address as *const ElfSymRaw32Bit) },
                endianness,
            ),
            Class::Bits64 => ElfSym::from_64bit(
                unsafe { read_unaligned(address as *const ElfSymRaw64Bit) },
                endianness,
            ),
        }
    }
}

impl RelocType {
    pub fn from(value: u32, machine: Machine) -> RelocType {
        match machine {
            Machine::X86 => RelocType::from_x86(value),
            Machine::X86_64 => RelocType::from_x86_64(value),
            Machine::AArch32 => RelocType::from_arm(value),
            Machine::AArch64 => RelocType::from_aarch64(value),
            Machine::RiscV => RelocType::from_riscv(value),
            _ => RelocType::Unknown(value),
        }
    }

    fn from_x86(value: u32) -> RelocType {
        use RelocType::*;
        match value {
            0 => None,
            1 => Absolute,
            2 => PcRel32,
            3 => Got32,
            4 => Plt32,
            5 => Copy,
            6 => GlobDat,
            7 => JumpSlot,
            8 => Relative,
            9 => GotOff,
            10 => GotPc,
            14 | 37 => TlsTpOff,
            15 | 16 => TlsInitialExec,
            17 | 34 => TlsLocalExec,
            18 => TlsGd,
            19 => TlsLd,
            32 => TlsLdo,
            35 => TlsDtpMod,
            36 => TlsDtpOff,
            41 => TlsDesc,
            42 => IRelative,
            43 => Got32X,
            n => Unknown(n),
        }
    }

    fn from_x86_64(value: u32) -> RelocType {
        use RelocType::*;
        match value {
            0 => None,
            1 => Absolute,
            2 => PcRel32,
            3 => Got32,
            4 => Plt32,
            5 => Copy,
            6 => GlobDat,
            7 => JumpSlot,
            8 => Relative,
            9 => GotPcRel,
            10 => Abs32ZeroExt,
            11 => Abs32SignExt,
            16 => TlsDtpMod,
            17 => TlsDtpOff,
            18 => TlsTpOff,
            19 => TlsGd,
            20 => TlsLd,
            21 => TlsLdo,
            22 => TlsInitialExec,
            23 => TlsLocalExec,
            36 => TlsDesc,
            37 => IRelative,
            41 => GotPcRelX,
            42 => RexGotPcRelX,
            n => Unknown(n),
        }
    }

    fn from_arm(value: u32) -> RelocType {
        use RelocType::*;
        match value {
            0 => None,
            2 => Absolute,
            3 => PcRel32,
            10 => ThumbCall,
            13 => TlsDesc,
            17 => TlsDtpMod,
            18 => TlsDtpOff,
            19 => TlsTpOff,
            20 => Copy,
            21 => GlobDat,
            22 => JumpSlot,
            23 => Relative,
            24 => GotOff,
            27 => Plt32,
            28 => Call,
            29 => Jump24,
            30 => ThumbJump24,
            38 => Target1,
            41 => Target2,
            42 => Prel31,
            43 => MovwAbsNc,
            44 => MovtAbs,
            47 => ThumbMovwAbsNc,
            48 => ThumbMovtAbs,
            104 => TlsGd,
            105 => TlsLd,
            106 => TlsLdo,
            107 => TlsInitialExec,
            108 => TlsLocalExec,
            160 => IRelative,
            n => Unknown(n),
        }
    }

    fn from_aarch64(value: u32) -> Self {
        use RelocType::*;
        match value {
            0 => None,
            257 | 258 => Absolute,
            260 => PcRel64,
            261 => PcRel32,
            274 => AdrPrelLo21,
            275 | 276 => AdrPage21,
            277 => AddAbsLo12,
            278 => Ldst8Lo12,
            279 => Tstbr14,
            280 => Condbr19,
            282 => Jump26,
            283 => Call,
            284 => Ldst16Lo12,
            285 => Ldst32Lo12,
            286 => Ldst64Lo12,
            299 => Ldst128Lo12,
            311 => AdrGotPage,
            312 => Ld64GotLo12,
            513 => TlsGdPage21,
            514 => TlsGdLo12,
            517 => TlsLdPage21,
            518 => TlsLdLo12,
            528 => TlsLdDtprelHi12,
            529 | 530 => TlsLdDtprelLo12,
            541 => TlsIePage21,
            542 => TlsIeLo12,
            549 => TlsLeTprelHi12,
            550 | 551 => TlsLeTprelLo12,
            562 => TlsDescPage21,
            563 => TlsDescLd64Lo12,
            564 => TlsDescAddLo12,
            569 => TlsDescCall,
            1024 => Copy,
            1025 => GlobDat,
            1026 => JumpSlot,
            1027 => Relative,
            1028 => TlsDtpMod,
            1029 => TlsDtpOff,
            1030 => TlsTpOff,
            1031 => TlsDesc,
            1032 => IRelative,
            n => Unknown(n),
        }
    }

    fn from_riscv(value: u32) -> RelocType {
        use RelocType::*;
        match value {
            0 => None,
            1 | 2 => Absolute,
            3 => Relative,
            4 => Copy,
            5 => JumpSlot,
            6 | 7 => TlsDtpMod,
            8 | 9 => TlsDtpOff,
            10 | 11 => TlsTpOff,
            12 => TlsDesc,
            16 => Branch,
            17 => Jal,
            18 => Call,
            19 => CallPlt,
            20 => GotHi20,
            21 => TlsGotHi20,
            22 => TlsGdHi20,
            23 => PcrelHi20,
            24 => PcrelLo12I,
            25 => PcrelLo12S,
            26 => Hi20,
            27 => Lo12I,
            28 => Lo12S,
            29 => TprelHi20,
            30 => TprelLo12I,
            31 => TprelLo12S,
            32 => TprelAdd,
            33 => Add8,
            34 => Add16,
            35 => Add32,
            36 => Add64,
            37 => Sub8,
            38 => Sub16,
            39 => Sub32,
            40 => Sub64,
            43 => Align,
            44 => RvcBranch,
            45 => RvcJump,
            51 => Relax,
            58 => IRelative,
            n => Unknown(n),
        }
    }
}

impl ElfRel {
    pub fn from_32bit(
        mut value: ElfRelRaw32Bit,
        endianness: Endianness,
        machine: Machine,
    ) -> ElfRel {
        value = value.correct_for_endianness(endianness);

        ElfRel {
            offset: value.offset as u64,
            symbol_index: value.info >> 8,
            r#type: RelocType::from(value.info & 0xff, machine),
        }
    }

    pub fn from_64bit(
        mut value: ElfRelRaw64Bit,
        endianness: Endianness,
        machine: Machine,
    ) -> ElfRel {
        value = value.correct_for_endianness(endianness);

        ElfRel {
            offset: value.offset,
            symbol_index: (value.info >> 32) as u32,
            r#type: RelocType::from(value.info as u32, machine),
        }
    }

    /// # Safety
    /// `address` must be readable for `size_of(ElfRelRaw32/64Bit)` bytes.
    pub unsafe fn parse_at(
        address: u64,
        class: Class,
        endianness: Endianness,
        machine: Machine,
    ) -> ElfRel {
        match class {
            Class::Bits32 => ElfRel::from_32bit(
                unsafe { read_unaligned(address as *const ElfRelRaw32Bit) },
                endianness,
                machine,
            ),
            Class::Bits64 => ElfRel::from_64bit(
                unsafe { read_unaligned(address as *const ElfRelRaw64Bit) },
                endianness,
                machine,
            ),
        }
    }
}

impl ElfRela {
    pub fn from_32bit(
        mut value: ElfRelaRaw32Bit,
        endianness: Endianness,
        machine: Machine,
    ) -> ElfRela {
        value = value.correct_for_endianness(endianness);

        ElfRela {
            offset: value.offset as u64,
            symbol_index: value.offset >> 8,
            r#type: RelocType::from(value.offset & 0xff, machine),
            addend: value.addend as u64,
        }
    }

    pub fn from_64bit(
        mut value: ElfRelaRaw64Bit,
        endianness: Endianness,
        machine: Machine,
    ) -> ElfRela {
        value = value.correct_for_endianness(endianness);

        ElfRela {
            offset: value.offset,
            symbol_index: (value.info >> 32) as u32,
            r#type: RelocType::from(value.info as u32, machine),
            addend: value.addend,
        }
    }

    /// # Safety
    /// `address` must be readable for `size_of(ElfRelaRaw32/64Bit)` bytes.
    pub unsafe fn parse_at(
        address: u64,
        class: Class,
        endianness: Endianness,
        machine: Machine,
    ) -> ElfRela {
        match class {
            Class::Bits32 => ElfRela::from_32bit(
                unsafe { read_unaligned(address as *const ElfRelaRaw32Bit) },
                endianness,
                machine,
            ),
            Class::Bits64 => ElfRela::from_64bit(
                unsafe { read_unaligned(address as *const ElfRelaRaw64Bit) },
                endianness,
                machine,
            ),
        }
    }
}
