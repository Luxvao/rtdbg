use rhai::export_module;
use rhai::{Dynamic, EvalAltResult};

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod class_module {

    use crate::Class;

    pub const Bits32: Class = Class::Bits32;
    pub const Bits64: Class = Class::Bits64;

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut Class) -> String {
        match value {
            Class::Bits32 => "Bits32".to_owned(),
            Class::Bits64 => "Bits64".to_owned(),
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod endianness_module {
    use crate::Endianness;

    pub const Little: Endianness = Endianness::Little;
    pub const Big: Endianness = Endianness::Big;

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut Endianness) -> String {
        match value {
            Endianness::Little => "Little".to_owned(),
            Endianness::Big => "Big".to_owned(),
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod osabi_module {
    use crate::OsAbi;

    pub const SystemV: OsAbi = OsAbi::SystemV;
    pub const HpUx: OsAbi = OsAbi::HpUx;
    pub const NetBSD: OsAbi = OsAbi::NetBSD;
    pub const Linux: OsAbi = OsAbi::Linux;
    pub const GNUHurd: OsAbi = OsAbi::GNUHurd;
    pub const Solaris: OsAbi = OsAbi::Solaris;
    pub const AIX: OsAbi = OsAbi::AIX;
    pub const IRIX: OsAbi = OsAbi::IRIX;
    pub const FreeBSD: OsAbi = OsAbi::FreeBSD;
    pub const Tru64: OsAbi = OsAbi::Tru64;
    pub const NovellModesto: OsAbi = OsAbi::NovellModesto;
    pub const OpenBSD: OsAbi = OsAbi::OpenBSD;
    pub const OpenVMS: OsAbi = OsAbi::OpenVMS;
    pub const NonStopKernel: OsAbi = OsAbi::NonStopKernel;
    pub const AROS: OsAbi = OsAbi::AROS;
    pub const FenixOS: OsAbi = OsAbi::FenixOS;
    pub const NuxiCloudABI: OsAbi = OsAbi::NuxiCloudABI;
    pub const StratusTechnologiesOpenVOS: OsAbi = OsAbi::StratusTechnologiesOpenVOS;

    pub fn Unknown(value: i64) -> OsAbi {
        OsAbi::Unknown(value as u8)
    }

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut OsAbi) -> String {
        match value {
            OsAbi::SystemV => "SystemV".to_owned(),
            OsAbi::HpUx => "HpUx".to_owned(),
            OsAbi::NetBSD => "NetBSD".to_owned(),
            OsAbi::Linux => "Linux".to_owned(),
            OsAbi::GNUHurd => "GNUHurd".to_owned(),
            OsAbi::Solaris => "Solaris".to_owned(),
            OsAbi::AIX => "AIX".to_owned(),
            OsAbi::IRIX => "IRIX".to_owned(),
            OsAbi::FreeBSD => "FreeBSD".to_owned(),
            OsAbi::Tru64 => "Tru64".to_owned(),
            OsAbi::NovellModesto => "NovellModesto".to_owned(),
            OsAbi::OpenBSD => "OpenBSD".to_owned(),
            OsAbi::OpenVMS => "OpenVMS".to_owned(),
            OsAbi::NonStopKernel => "NonStopKernel".to_owned(),
            OsAbi::AROS => "AROS".to_owned(),
            OsAbi::FenixOS => "FenixOS".to_owned(),
            OsAbi::NuxiCloudABI => "NuxiCloudABI".to_owned(),
            OsAbi::StratusTechnologiesOpenVOS => "StratusTechnologiesOpenVOS".to_owned(),
            OsAbi::Unknown(_) => "Unknown".to_owned(),
        }
    }

    #[rhai_fn(global, get = "field_0", pure)]
    pub fn get_field_0(value: &mut OsAbi) -> Dynamic {
        match value {
            OsAbi::SystemV => Dynamic::UNIT,
            OsAbi::HpUx => Dynamic::UNIT,
            OsAbi::NetBSD => Dynamic::UNIT,
            OsAbi::Linux => Dynamic::UNIT,
            OsAbi::GNUHurd => Dynamic::UNIT,
            OsAbi::Solaris => Dynamic::UNIT,
            OsAbi::AIX => Dynamic::UNIT,
            OsAbi::IRIX => Dynamic::UNIT,
            OsAbi::FreeBSD => Dynamic::UNIT,
            OsAbi::Tru64 => Dynamic::UNIT,
            OsAbi::NovellModesto => Dynamic::UNIT,
            OsAbi::OpenBSD => Dynamic::UNIT,
            OsAbi::OpenVMS => Dynamic::UNIT,
            OsAbi::NonStopKernel => Dynamic::UNIT,
            OsAbi::AROS => Dynamic::UNIT,
            OsAbi::FenixOS => Dynamic::UNIT,
            OsAbi::NuxiCloudABI => Dynamic::UNIT,
            OsAbi::StratusTechnologiesOpenVOS => Dynamic::UNIT,
            OsAbi::Unknown(raw) => Dynamic::from(*raw as i64),
        }
    }

    #[rhai_fn(global, set = "field_0", return_raw)]
    pub fn set_field_0(value: &mut OsAbi, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            OsAbi::SystemV => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::HpUx => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::NetBSD => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::Linux => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::GNUHurd => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::Solaris => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::AIX => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::IRIX => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::FreeBSD => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::Tru64 => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::NovellModesto => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::OpenBSD => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::OpenVMS => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::NonStopKernel => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::AROS => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::FenixOS => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::NuxiCloudABI => Err("field_0 is only writable on Unknown variants".into()),
            OsAbi::StratusTechnologiesOpenVOS => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            OsAbi::Unknown(raw) => {
                *raw = new_value as u8;
                Ok(())
            }
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod elftype_module {
    use crate::ElfType;

    use super::*;
    pub const EtNone: ElfType = ElfType::EtNone;
    pub const EtRel: ElfType = ElfType::EtRel;
    pub const EtExec: ElfType = ElfType::EtExec;
    pub const EtDyn: ElfType = ElfType::EtDyn;
    pub const EtCore: ElfType = ElfType::EtCore;
    pub const EtLoos: ElfType = ElfType::EtLoos;
    pub const EtHios: ElfType = ElfType::EtHios;
    pub const EtLoproc: ElfType = ElfType::EtLoproc;
    pub const EtHiproc: ElfType = ElfType::EtHiproc;

    pub fn Unknown(value: i64) -> ElfType {
        ElfType::Unknown(value as u16)
    }

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut ElfType) -> String {
        match value {
            ElfType::EtNone => "EtNone".to_owned(),
            ElfType::EtRel => "EtRel".to_owned(),
            ElfType::EtExec => "EtExec".to_owned(),
            ElfType::EtDyn => "EtDyn".to_owned(),
            ElfType::EtCore => "EtCore".to_owned(),
            ElfType::EtLoos => "EtLoos".to_owned(),
            ElfType::EtHios => "EtHios".to_owned(),
            ElfType::EtLoproc => "EtLoproc".to_owned(),
            ElfType::EtHiproc => "EtHiproc".to_owned(),
            ElfType::Unknown(_) => "Unknown".to_owned(),
        }
    }

    #[rhai_fn(global, get = "field_0", pure)]
    pub fn get_field_0(value: &mut ElfType) -> Dynamic {
        match value {
            ElfType::EtNone => Dynamic::UNIT,
            ElfType::EtRel => Dynamic::UNIT,
            ElfType::EtExec => Dynamic::UNIT,
            ElfType::EtDyn => Dynamic::UNIT,
            ElfType::EtCore => Dynamic::UNIT,
            ElfType::EtLoos => Dynamic::UNIT,
            ElfType::EtHios => Dynamic::UNIT,
            ElfType::EtLoproc => Dynamic::UNIT,
            ElfType::EtHiproc => Dynamic::UNIT,
            ElfType::Unknown(raw) => Dynamic::from(*raw as i64),
        }
    }

    #[rhai_fn(global, set = "field_0", return_raw)]
    pub fn set_field_0(value: &mut ElfType, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            ElfType::EtNone => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtRel => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtExec => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtDyn => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtCore => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtLoos => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtHios => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtLoproc => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::EtHiproc => Err("field_0 is only writable on Unknown variants".into()),
            ElfType::Unknown(raw) => {
                *raw = new_value as u16;
                Ok(())
            }
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod machine_module {
    use crate::Machine;

    pub const None: Machine = Machine::None;
    pub const AtNtWe32100: Machine = Machine::AtNtWe32100;
    pub const Sparc: Machine = Machine::Sparc;
    pub const X86: Machine = Machine::X86;
    pub const Motorola68k: Machine = Machine::Motorola68k;
    pub const Motorola88k: Machine = Machine::Motorola88k;
    pub const IntelMcu: Machine = Machine::IntelMcu;
    pub const Intel80860: Machine = Machine::Intel80860;
    pub const Mips: Machine = Machine::Mips;
    pub const IbmSystem370: Machine = Machine::IbmSystem370;
    pub const MipsRs3000LE: Machine = Machine::MipsRs3000LE;
    pub const HpPaRisc: Machine = Machine::HpPaRisc;
    pub const Intel80960: Machine = Machine::Intel80960;
    pub const PowerPc: Machine = Machine::PowerPc;
    pub const PowerPc64: Machine = Machine::PowerPc64;
    pub const S390x: Machine = Machine::S390x;
    pub const IbmSpc: Machine = Machine::IbmSpc;
    pub const NecV800: Machine = Machine::NecV800;
    pub const FujistuFr20: Machine = Machine::FujistuFr20;
    pub const TrwRh32: Machine = Machine::TrwRh32;
    pub const MotorolaRce: Machine = Machine::MotorolaRce;
    pub const AArch32: Machine = Machine::AArch32;
    pub const DigitalAlpha: Machine = Machine::DigitalAlpha;
    pub const SuperH: Machine = Machine::SuperH;
    pub const SparcV9: Machine = Machine::SparcV9;
    pub const SiemensTriCoreEP: Machine = Machine::SiemensTriCoreEP;
    pub const ArgonautRiscCore: Machine = Machine::ArgonautRiscCore;
    pub const Hitachi300: Machine = Machine::Hitachi300;
    pub const Hitachi300H: Machine = Machine::Hitachi300H;
    pub const HitachiH8S: Machine = Machine::HitachiH8S;
    pub const Hitachi500: Machine = Machine::Hitachi500;
    pub const Ia64: Machine = Machine::Ia64;
    pub const StanfordMipsX: Machine = Machine::StanfordMipsX;
    pub const MotorolaColdFire: Machine = Machine::MotorolaColdFire;
    pub const MotorolaM68HC12: Machine = Machine::MotorolaM68HC12;
    pub const FujitsuMmaMultimediaAccelerator: Machine = Machine::FujitsuMmaMultimediaAccelerator;
    pub const SiemensPcp: Machine = Machine::SiemensPcp;
    pub const SonyNCpuERiscP: Machine = Machine::SonyNCpuERiscP;
    pub const DensoNdr1Mcp: Machine = Machine::DensoNdr1Mcp;
    pub const MotorolaStarCoreP: Machine = Machine::MotorolaStarCoreP;
    pub const ToyotaMe16P: Machine = Machine::ToyotaMe16P;
    pub const STMicroelectronicsSt100P: Machine = Machine::STMicroelectronicsSt100P;
    pub const TinyJ: Machine = Machine::TinyJ;
    pub const X86_64: Machine = Machine::X86_64;
    pub const SonyDspP: Machine = Machine::SonyDspP;
    pub const DigitalEquipmentCorpPdp10: Machine = Machine::DigitalEquipmentCorpPdp10;
    pub const DigitalEquipmentCorpPdp11: Machine = Machine::DigitalEquipmentCorpPdp11;
    pub const SiemensFx66Mcu: Machine = Machine::SiemensFx66Mcu;
    pub const STMicroelectronicsSt9: Machine = Machine::STMicroelectronicsSt9;
    pub const StMicroelectronicsSt7: Machine = Machine::StMicroelectronicsSt7;
    pub const MotorolaMC68HC16Mcu: Machine = Machine::MotorolaMC68HC16Mcu;
    pub const MotorolaMC68HC11Mcu: Machine = Machine::MotorolaMC68HC11Mcu;
    pub const MotorolaMC68HC08: Machine = Machine::MotorolaMC68HC08;
    pub const MotorolaMC68HC05Mcu: Machine = Machine::MotorolaMC68HC05Mcu;
    pub const SiliconGraphicsSvX: Machine = Machine::SiliconGraphicsSvX;
    pub const STMicroelectronicsSt19: Machine = Machine::STMicroelectronicsSt19;
    pub const DigitalVax: Machine = Machine::DigitalVax;
    pub const AxisCommuncationsMcp: Machine = Machine::AxisCommuncationsMcp;
    pub const InfineonTechnologiesMcp: Machine = Machine::InfineonTechnologiesMcp;
    pub const Element14DspP: Machine = Machine::Element14DspP;
    pub const LsiLogicDspP: Machine = Machine::LsiLogicDspP;
    pub const TMS320C6000: Machine = Machine::TMS320C6000;
    pub const McstElbrusE2K: Machine = Machine::McstElbrusE2K;
    pub const AArch64: Machine = Machine::AArch64;
    pub const ZilogZ80: Machine = Machine::ZilogZ80;
    pub const RiscV: Machine = Machine::RiscV;
    pub const BerkeleyPacketFilter: Machine = Machine::BerkeleyPacketFilter;
    pub const Wdc65C816: Machine = Machine::Wdc65C816;
    pub const LoongArch: Machine = Machine::LoongArch;

    pub fn Unknown(value: i64) -> Machine {
        Machine::Unknown(value as u16)
    }

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut Machine) -> String {
        match value {
            Machine::None => "None".to_owned(),
            Machine::AtNtWe32100 => "AtNtWe32100".to_owned(),
            Machine::Sparc => "Sparc".to_owned(),
            Machine::X86 => "X86".to_owned(),
            Machine::Motorola68k => "Motorola68k".to_owned(),
            Machine::Motorola88k => "Motorola88k".to_owned(),
            Machine::IntelMcu => "IntelMcu".to_owned(),
            Machine::Intel80860 => "Intel80860".to_owned(),
            Machine::Mips => "Mips".to_owned(),
            Machine::IbmSystem370 => "IbmSystem370".to_owned(),
            Machine::MipsRs3000LE => "MipsRs3000LE".to_owned(),
            Machine::HpPaRisc => "HpPaRisc".to_owned(),
            Machine::Intel80960 => "Intel80960".to_owned(),
            Machine::PowerPc => "PowerPc".to_owned(),
            Machine::PowerPc64 => "PowerPc64".to_owned(),
            Machine::S390x => "S390x".to_owned(),
            Machine::IbmSpc => "IbmSpc".to_owned(),
            Machine::NecV800 => "NecV800".to_owned(),
            Machine::FujistuFr20 => "FujistuFr20".to_owned(),
            Machine::TrwRh32 => "TrwRh32".to_owned(),
            Machine::MotorolaRce => "MotorolaRce".to_owned(),
            Machine::AArch32 => "AArch32".to_owned(),
            Machine::DigitalAlpha => "DigitalAlpha".to_owned(),
            Machine::SuperH => "SuperH".to_owned(),
            Machine::SparcV9 => "SparcV9".to_owned(),
            Machine::SiemensTriCoreEP => "SiemensTriCoreEP".to_owned(),
            Machine::ArgonautRiscCore => "ArgonautRiscCore".to_owned(),
            Machine::Hitachi300 => "Hitachi300".to_owned(),
            Machine::Hitachi300H => "Hitachi300H".to_owned(),
            Machine::HitachiH8S => "HitachiH8S".to_owned(),
            Machine::Hitachi500 => "Hitachi500".to_owned(),
            Machine::Ia64 => "Ia64".to_owned(),
            Machine::StanfordMipsX => "StanfordMipsX".to_owned(),
            Machine::MotorolaColdFire => "MotorolaColdFire".to_owned(),
            Machine::MotorolaM68HC12 => "MotorolaM68HC12".to_owned(),
            Machine::FujitsuMmaMultimediaAccelerator => {
                "FujitsuMmaMultimediaAccelerator".to_owned()
            }
            Machine::SiemensPcp => "SiemensPcp".to_owned(),
            Machine::SonyNCpuERiscP => "SonyNCpuERiscP".to_owned(),
            Machine::DensoNdr1Mcp => "DensoNdr1Mcp".to_owned(),
            Machine::MotorolaStarCoreP => "MotorolaStarCoreP".to_owned(),
            Machine::ToyotaMe16P => "ToyotaMe16P".to_owned(),
            Machine::STMicroelectronicsSt100P => "STMicroelectronicsSt100P".to_owned(),
            Machine::TinyJ => "TinyJ".to_owned(),
            Machine::X86_64 => "X86_64".to_owned(),
            Machine::SonyDspP => "SonyDspP".to_owned(),
            Machine::DigitalEquipmentCorpPdp10 => "DigitalEquipmentCorpPdp10".to_owned(),
            Machine::DigitalEquipmentCorpPdp11 => "DigitalEquipmentCorpPdp11".to_owned(),
            Machine::SiemensFx66Mcu => "SiemensFx66Mcu".to_owned(),
            Machine::STMicroelectronicsSt9 => "STMicroelectronicsSt9".to_owned(),
            Machine::StMicroelectronicsSt7 => "StMicroelectronicsSt7".to_owned(),
            Machine::MotorolaMC68HC16Mcu => "MotorolaMC68HC16Mcu".to_owned(),
            Machine::MotorolaMC68HC11Mcu => "MotorolaMC68HC11Mcu".to_owned(),
            Machine::MotorolaMC68HC08 => "MotorolaMC68HC08".to_owned(),
            Machine::MotorolaMC68HC05Mcu => "MotorolaMC68HC05Mcu".to_owned(),
            Machine::SiliconGraphicsSvX => "SiliconGraphicsSvX".to_owned(),
            Machine::STMicroelectronicsSt19 => "STMicroelectronicsSt19".to_owned(),
            Machine::DigitalVax => "DigitalVax".to_owned(),
            Machine::AxisCommuncationsMcp => "AxisCommuncationsMcp".to_owned(),
            Machine::InfineonTechnologiesMcp => "InfineonTechnologiesMcp".to_owned(),
            Machine::Element14DspP => "Element14DspP".to_owned(),
            Machine::LsiLogicDspP => "LsiLogicDspP".to_owned(),
            Machine::TMS320C6000 => "TMS320C6000".to_owned(),
            Machine::McstElbrusE2K => "McstElbrusE2K".to_owned(),
            Machine::AArch64 => "AArch64".to_owned(),
            Machine::ZilogZ80 => "ZilogZ80".to_owned(),
            Machine::RiscV => "RiscV".to_owned(),
            Machine::BerkeleyPacketFilter => "BerkeleyPacketFilter".to_owned(),
            Machine::Wdc65C816 => "Wdc65C816".to_owned(),
            Machine::LoongArch => "LoongArch".to_owned(),
            Machine::Unknown(_) => "Unknown".to_owned(),
        }
    }

    #[rhai_fn(global, get = "field_0", pure)]
    pub fn get_field_0(value: &mut Machine) -> Dynamic {
        match value {
            Machine::None => Dynamic::UNIT,
            Machine::AtNtWe32100 => Dynamic::UNIT,
            Machine::Sparc => Dynamic::UNIT,
            Machine::X86 => Dynamic::UNIT,
            Machine::Motorola68k => Dynamic::UNIT,
            Machine::Motorola88k => Dynamic::UNIT,
            Machine::IntelMcu => Dynamic::UNIT,
            Machine::Intel80860 => Dynamic::UNIT,
            Machine::Mips => Dynamic::UNIT,
            Machine::IbmSystem370 => Dynamic::UNIT,
            Machine::MipsRs3000LE => Dynamic::UNIT,
            Machine::HpPaRisc => Dynamic::UNIT,
            Machine::Intel80960 => Dynamic::UNIT,
            Machine::PowerPc => Dynamic::UNIT,
            Machine::PowerPc64 => Dynamic::UNIT,
            Machine::S390x => Dynamic::UNIT,
            Machine::IbmSpc => Dynamic::UNIT,
            Machine::NecV800 => Dynamic::UNIT,
            Machine::FujistuFr20 => Dynamic::UNIT,
            Machine::TrwRh32 => Dynamic::UNIT,
            Machine::MotorolaRce => Dynamic::UNIT,
            Machine::AArch32 => Dynamic::UNIT,
            Machine::DigitalAlpha => Dynamic::UNIT,
            Machine::SuperH => Dynamic::UNIT,
            Machine::SparcV9 => Dynamic::UNIT,
            Machine::SiemensTriCoreEP => Dynamic::UNIT,
            Machine::ArgonautRiscCore => Dynamic::UNIT,
            Machine::Hitachi300 => Dynamic::UNIT,
            Machine::Hitachi300H => Dynamic::UNIT,
            Machine::HitachiH8S => Dynamic::UNIT,
            Machine::Hitachi500 => Dynamic::UNIT,
            Machine::Ia64 => Dynamic::UNIT,
            Machine::StanfordMipsX => Dynamic::UNIT,
            Machine::MotorolaColdFire => Dynamic::UNIT,
            Machine::MotorolaM68HC12 => Dynamic::UNIT,
            Machine::FujitsuMmaMultimediaAccelerator => Dynamic::UNIT,
            Machine::SiemensPcp => Dynamic::UNIT,
            Machine::SonyNCpuERiscP => Dynamic::UNIT,
            Machine::DensoNdr1Mcp => Dynamic::UNIT,
            Machine::MotorolaStarCoreP => Dynamic::UNIT,
            Machine::ToyotaMe16P => Dynamic::UNIT,
            Machine::STMicroelectronicsSt100P => Dynamic::UNIT,
            Machine::TinyJ => Dynamic::UNIT,
            Machine::X86_64 => Dynamic::UNIT,
            Machine::SonyDspP => Dynamic::UNIT,
            Machine::DigitalEquipmentCorpPdp10 => Dynamic::UNIT,
            Machine::DigitalEquipmentCorpPdp11 => Dynamic::UNIT,
            Machine::SiemensFx66Mcu => Dynamic::UNIT,
            Machine::STMicroelectronicsSt9 => Dynamic::UNIT,
            Machine::StMicroelectronicsSt7 => Dynamic::UNIT,
            Machine::MotorolaMC68HC16Mcu => Dynamic::UNIT,
            Machine::MotorolaMC68HC11Mcu => Dynamic::UNIT,
            Machine::MotorolaMC68HC08 => Dynamic::UNIT,
            Machine::MotorolaMC68HC05Mcu => Dynamic::UNIT,
            Machine::SiliconGraphicsSvX => Dynamic::UNIT,
            Machine::STMicroelectronicsSt19 => Dynamic::UNIT,
            Machine::DigitalVax => Dynamic::UNIT,
            Machine::AxisCommuncationsMcp => Dynamic::UNIT,
            Machine::InfineonTechnologiesMcp => Dynamic::UNIT,
            Machine::Element14DspP => Dynamic::UNIT,
            Machine::LsiLogicDspP => Dynamic::UNIT,
            Machine::TMS320C6000 => Dynamic::UNIT,
            Machine::McstElbrusE2K => Dynamic::UNIT,
            Machine::AArch64 => Dynamic::UNIT,
            Machine::ZilogZ80 => Dynamic::UNIT,
            Machine::RiscV => Dynamic::UNIT,
            Machine::BerkeleyPacketFilter => Dynamic::UNIT,
            Machine::Wdc65C816 => Dynamic::UNIT,
            Machine::LoongArch => Dynamic::UNIT,
            Machine::Unknown(raw) => Dynamic::from(*raw as i64),
        }
    }

    #[rhai_fn(global, set = "field_0", return_raw)]
    pub fn set_field_0(value: &mut Machine, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            Machine::None => Err("field_0 is only writable on Unknown variants".into()),
            Machine::AtNtWe32100 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Sparc => Err("field_0 is only writable on Unknown variants".into()),
            Machine::X86 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Motorola68k => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Motorola88k => Err("field_0 is only writable on Unknown variants".into()),
            Machine::IntelMcu => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Intel80860 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Mips => Err("field_0 is only writable on Unknown variants".into()),
            Machine::IbmSystem370 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::MipsRs3000LE => Err("field_0 is only writable on Unknown variants".into()),
            Machine::HpPaRisc => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Intel80960 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::PowerPc => Err("field_0 is only writable on Unknown variants".into()),
            Machine::PowerPc64 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::S390x => Err("field_0 is only writable on Unknown variants".into()),
            Machine::IbmSpc => Err("field_0 is only writable on Unknown variants".into()),
            Machine::NecV800 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::FujistuFr20 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::TrwRh32 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::MotorolaRce => Err("field_0 is only writable on Unknown variants".into()),
            Machine::AArch32 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::DigitalAlpha => Err("field_0 is only writable on Unknown variants".into()),
            Machine::SuperH => Err("field_0 is only writable on Unknown variants".into()),
            Machine::SparcV9 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::SiemensTriCoreEP => Err("field_0 is only writable on Unknown variants".into()),
            Machine::ArgonautRiscCore => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Hitachi300 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Hitachi300H => Err("field_0 is only writable on Unknown variants".into()),
            Machine::HitachiH8S => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Hitachi500 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Ia64 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::StanfordMipsX => Err("field_0 is only writable on Unknown variants".into()),
            Machine::MotorolaColdFire => Err("field_0 is only writable on Unknown variants".into()),
            Machine::MotorolaM68HC12 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::FujitsuMmaMultimediaAccelerator => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::SiemensPcp => Err("field_0 is only writable on Unknown variants".into()),
            Machine::SonyNCpuERiscP => Err("field_0 is only writable on Unknown variants".into()),
            Machine::DensoNdr1Mcp => Err("field_0 is only writable on Unknown variants".into()),
            Machine::MotorolaStarCoreP => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::ToyotaMe16P => Err("field_0 is only writable on Unknown variants".into()),
            Machine::STMicroelectronicsSt100P => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::TinyJ => Err("field_0 is only writable on Unknown variants".into()),
            Machine::X86_64 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::SonyDspP => Err("field_0 is only writable on Unknown variants".into()),
            Machine::DigitalEquipmentCorpPdp10 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::DigitalEquipmentCorpPdp11 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::SiemensFx66Mcu => Err("field_0 is only writable on Unknown variants".into()),
            Machine::STMicroelectronicsSt9 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::StMicroelectronicsSt7 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::MotorolaMC68HC16Mcu => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::MotorolaMC68HC11Mcu => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::MotorolaMC68HC08 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::MotorolaMC68HC05Mcu => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::SiliconGraphicsSvX => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::STMicroelectronicsSt19 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::DigitalVax => Err("field_0 is only writable on Unknown variants".into()),
            Machine::AxisCommuncationsMcp => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::InfineonTechnologiesMcp => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::Element14DspP => Err("field_0 is only writable on Unknown variants".into()),
            Machine::LsiLogicDspP => Err("field_0 is only writable on Unknown variants".into()),
            Machine::TMS320C6000 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::McstElbrusE2K => Err("field_0 is only writable on Unknown variants".into()),
            Machine::AArch64 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::ZilogZ80 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::RiscV => Err("field_0 is only writable on Unknown variants".into()),
            Machine::BerkeleyPacketFilter => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            Machine::Wdc65C816 => Err("field_0 is only writable on Unknown variants".into()),
            Machine::LoongArch => Err("field_0 is only writable on Unknown variants".into()),
            Machine::Unknown(raw) => {
                *raw = new_value as u16;
                Ok(())
            }
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod programtype_module {
    use crate::ProgramType;

    pub const PtNull: ProgramType = ProgramType::PtNull;
    pub const PtLoad: ProgramType = ProgramType::PtLoad;
    pub const PtDynamic: ProgramType = ProgramType::PtDynamic;
    pub const PtInterp: ProgramType = ProgramType::PtInterp;
    pub const PtNote: ProgramType = ProgramType::PtNote;
    pub const PtShlib: ProgramType = ProgramType::PtShlib;
    pub const PtPhdr: ProgramType = ProgramType::PtPhdr;
    pub const PtTls: ProgramType = ProgramType::PtTls;
    pub const PtLoos: ProgramType = ProgramType::PtLoos;
    pub const PtHios: ProgramType = ProgramType::PtHios;
    pub const PtLoproc: ProgramType = ProgramType::PtLoproc;
    pub const PtHiproc: ProgramType = ProgramType::PtHiproc;

    pub fn Unknown(value: i64) -> ProgramType {
        ProgramType::Unknown(value as u32)
    }

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut ProgramType) -> String {
        match value {
            ProgramType::PtNull => "PtNull".to_owned(),
            ProgramType::PtLoad => "PtLoad".to_owned(),
            ProgramType::PtDynamic => "PtDynamic".to_owned(),
            ProgramType::PtInterp => "PtInterp".to_owned(),
            ProgramType::PtNote => "PtNote".to_owned(),
            ProgramType::PtShlib => "PtShlib".to_owned(),
            ProgramType::PtPhdr => "PtPhdr".to_owned(),
            ProgramType::PtTls => "PtTls".to_owned(),
            ProgramType::PtLoos => "PtLoos".to_owned(),
            ProgramType::PtHios => "PtHios".to_owned(),
            ProgramType::PtLoproc => "PtLoproc".to_owned(),
            ProgramType::PtHiproc => "PtHiproc".to_owned(),
            ProgramType::Unknown(_) => "Unknown".to_owned(),
        }
    }

    #[rhai_fn(global, get = "field_0", pure)]
    pub fn get_field_0(value: &mut ProgramType) -> Dynamic {
        match value {
            ProgramType::PtNull => Dynamic::UNIT,
            ProgramType::PtLoad => Dynamic::UNIT,
            ProgramType::PtDynamic => Dynamic::UNIT,
            ProgramType::PtInterp => Dynamic::UNIT,
            ProgramType::PtNote => Dynamic::UNIT,
            ProgramType::PtShlib => Dynamic::UNIT,
            ProgramType::PtPhdr => Dynamic::UNIT,
            ProgramType::PtTls => Dynamic::UNIT,
            ProgramType::PtLoos => Dynamic::UNIT,
            ProgramType::PtHios => Dynamic::UNIT,
            ProgramType::PtLoproc => Dynamic::UNIT,
            ProgramType::PtHiproc => Dynamic::UNIT,
            ProgramType::Unknown(raw) => Dynamic::from(*raw as i64),
        }
    }

    #[rhai_fn(global, set = "field_0", return_raw)]
    pub fn set_field_0(value: &mut ProgramType, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            ProgramType::PtNull => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtLoad => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtDynamic => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtInterp => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtNote => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtShlib => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtPhdr => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtTls => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtLoos => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtHios => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtLoproc => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::PtHiproc => Err("field_0 is only writable on Unknown variants".into()),
            ProgramType::Unknown(raw) => {
                *raw = new_value as u32;
                Ok(())
            }
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod reloctype_module {
    use crate::RelocType;

    pub const None: RelocType = RelocType::None;
    pub const Relative: RelocType = RelocType::Relative;
    pub const Absolute: RelocType = RelocType::Absolute;
    pub const GlobDat: RelocType = RelocType::GlobDat;
    pub const JumpSlot: RelocType = RelocType::JumpSlot;
    pub const Copy: RelocType = RelocType::Copy;
    pub const IRelative: RelocType = RelocType::IRelative;
    pub const TlsDtpMod: RelocType = RelocType::TlsDtpMod;
    pub const TlsDtpOff: RelocType = RelocType::TlsDtpOff;
    pub const TlsTpOff: RelocType = RelocType::TlsTpOff;
    pub const TlsDesc: RelocType = RelocType::TlsDesc;
    pub const PcRel32: RelocType = RelocType::PcRel32;
    pub const PcRel64: RelocType = RelocType::PcRel64;
    pub const Plt32: RelocType = RelocType::Plt32;
    pub const Got32: RelocType = RelocType::Got32;
    pub const Got32X: RelocType = RelocType::Got32X;
    pub const GotOff: RelocType = RelocType::GotOff;
    pub const GotPc: RelocType = RelocType::GotPc;
    pub const GotPcRel: RelocType = RelocType::GotPcRel;
    pub const GotPcRelX: RelocType = RelocType::GotPcRelX;
    pub const RexGotPcRelX: RelocType = RelocType::RexGotPcRelX;
    pub const Abs32ZeroExt: RelocType = RelocType::Abs32ZeroExt;
    pub const Abs32SignExt: RelocType = RelocType::Abs32SignExt;
    pub const TlsGd: RelocType = RelocType::TlsGd;
    pub const TlsLd: RelocType = RelocType::TlsLd;
    pub const TlsLdo: RelocType = RelocType::TlsLdo;
    pub const TlsInitialExec: RelocType = RelocType::TlsInitialExec;
    pub const TlsLocalExec: RelocType = RelocType::TlsLocalExec;
    pub const Call: RelocType = RelocType::Call;
    pub const CallPlt: RelocType = RelocType::CallPlt;
    pub const Jump24: RelocType = RelocType::Jump24;
    pub const Jump26: RelocType = RelocType::Jump26;
    pub const ThumbCall: RelocType = RelocType::ThumbCall;
    pub const ThumbJump24: RelocType = RelocType::ThumbJump24;
    pub const Branch: RelocType = RelocType::Branch;
    pub const Jal: RelocType = RelocType::Jal;
    pub const RvcBranch: RelocType = RelocType::RvcBranch;
    pub const RvcJump: RelocType = RelocType::RvcJump;
    pub const MovwAbsNc: RelocType = RelocType::MovwAbsNc;
    pub const MovtAbs: RelocType = RelocType::MovtAbs;
    pub const ThumbMovwAbsNc: RelocType = RelocType::ThumbMovwAbsNc;
    pub const ThumbMovtAbs: RelocType = RelocType::ThumbMovtAbs;
    pub const Prel31: RelocType = RelocType::Prel31;
    pub const Target1: RelocType = RelocType::Target1;
    pub const Target2: RelocType = RelocType::Target2;
    pub const AdrPrelLo21: RelocType = RelocType::AdrPrelLo21;
    pub const AdrPage21: RelocType = RelocType::AdrPage21;
    pub const AddAbsLo12: RelocType = RelocType::AddAbsLo12;
    pub const Ldst8Lo12: RelocType = RelocType::Ldst8Lo12;
    pub const Ldst16Lo12: RelocType = RelocType::Ldst16Lo12;
    pub const Ldst32Lo12: RelocType = RelocType::Ldst32Lo12;
    pub const Ldst64Lo12: RelocType = RelocType::Ldst64Lo12;
    pub const Ldst128Lo12: RelocType = RelocType::Ldst128Lo12;
    pub const Tstbr14: RelocType = RelocType::Tstbr14;
    pub const Condbr19: RelocType = RelocType::Condbr19;
    pub const AdrGotPage: RelocType = RelocType::AdrGotPage;
    pub const Ld64GotLo12: RelocType = RelocType::Ld64GotLo12;
    pub const TlsGdPage21: RelocType = RelocType::TlsGdPage21;
    pub const TlsGdLo12: RelocType = RelocType::TlsGdLo12;
    pub const TlsLdPage21: RelocType = RelocType::TlsLdPage21;
    pub const TlsLdLo12: RelocType = RelocType::TlsLdLo12;
    pub const TlsLdDtprelHi12: RelocType = RelocType::TlsLdDtprelHi12;
    pub const TlsLdDtprelLo12: RelocType = RelocType::TlsLdDtprelLo12;
    pub const TlsIePage21: RelocType = RelocType::TlsIePage21;
    pub const TlsIeLo12: RelocType = RelocType::TlsIeLo12;
    pub const TlsLeTprelHi12: RelocType = RelocType::TlsLeTprelHi12;
    pub const TlsLeTprelLo12: RelocType = RelocType::TlsLeTprelLo12;
    pub const TlsDescPage21: RelocType = RelocType::TlsDescPage21;
    pub const TlsDescLd64Lo12: RelocType = RelocType::TlsDescLd64Lo12;
    pub const TlsDescAddLo12: RelocType = RelocType::TlsDescAddLo12;
    pub const TlsDescCall: RelocType = RelocType::TlsDescCall;
    pub const GotHi20: RelocType = RelocType::GotHi20;
    pub const PcrelHi20: RelocType = RelocType::PcrelHi20;
    pub const PcrelLo12I: RelocType = RelocType::PcrelLo12I;
    pub const PcrelLo12S: RelocType = RelocType::PcrelLo12S;
    pub const Hi20: RelocType = RelocType::Hi20;
    pub const Lo12I: RelocType = RelocType::Lo12I;
    pub const Lo12S: RelocType = RelocType::Lo12S;
    pub const TlsGotHi20: RelocType = RelocType::TlsGotHi20;
    pub const TlsGdHi20: RelocType = RelocType::TlsGdHi20;
    pub const TprelHi20: RelocType = RelocType::TprelHi20;
    pub const TprelLo12I: RelocType = RelocType::TprelLo12I;
    pub const TprelLo12S: RelocType = RelocType::TprelLo12S;
    pub const TprelAdd: RelocType = RelocType::TprelAdd;
    pub const Relax: RelocType = RelocType::Relax;
    pub const Align: RelocType = RelocType::Align;
    pub const Add8: RelocType = RelocType::Add8;
    pub const Add16: RelocType = RelocType::Add16;
    pub const Add32: RelocType = RelocType::Add32;
    pub const Add64: RelocType = RelocType::Add64;
    pub const Sub8: RelocType = RelocType::Sub8;
    pub const Sub16: RelocType = RelocType::Sub16;
    pub const Sub32: RelocType = RelocType::Sub32;
    pub const Sub64: RelocType = RelocType::Sub64;

    pub fn Unknown(value: i64) -> RelocType {
        RelocType::Unknown(value as u32)
    }

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut RelocType) -> String {
        match value {
            RelocType::None => "None".to_owned(),
            RelocType::Relative => "Relative".to_owned(),
            RelocType::Absolute => "Absolute".to_owned(),
            RelocType::GlobDat => "GlobDat".to_owned(),
            RelocType::JumpSlot => "JumpSlot".to_owned(),
            RelocType::Copy => "Copy".to_owned(),
            RelocType::IRelative => "IRelative".to_owned(),
            RelocType::TlsDtpMod => "TlsDtpMod".to_owned(),
            RelocType::TlsDtpOff => "TlsDtpOff".to_owned(),
            RelocType::TlsTpOff => "TlsTpOff".to_owned(),
            RelocType::TlsDesc => "TlsDesc".to_owned(),
            RelocType::PcRel32 => "PcRel32".to_owned(),
            RelocType::PcRel64 => "PcRel64".to_owned(),
            RelocType::Plt32 => "Plt32".to_owned(),
            RelocType::Got32 => "Got32".to_owned(),
            RelocType::Got32X => "Got32X".to_owned(),
            RelocType::GotOff => "GotOff".to_owned(),
            RelocType::GotPc => "GotPc".to_owned(),
            RelocType::GotPcRel => "GotPcRel".to_owned(),
            RelocType::GotPcRelX => "GotPcRelX".to_owned(),
            RelocType::RexGotPcRelX => "RexGotPcRelX".to_owned(),
            RelocType::Abs32ZeroExt => "Abs32ZeroExt".to_owned(),
            RelocType::Abs32SignExt => "Abs32SignExt".to_owned(),
            RelocType::TlsGd => "TlsGd".to_owned(),
            RelocType::TlsLd => "TlsLd".to_owned(),
            RelocType::TlsLdo => "TlsLdo".to_owned(),
            RelocType::TlsInitialExec => "TlsInitialExec".to_owned(),
            RelocType::TlsLocalExec => "TlsLocalExec".to_owned(),
            RelocType::Call => "Call".to_owned(),
            RelocType::CallPlt => "CallPlt".to_owned(),
            RelocType::Jump24 => "Jump24".to_owned(),
            RelocType::Jump26 => "Jump26".to_owned(),
            RelocType::ThumbCall => "ThumbCall".to_owned(),
            RelocType::ThumbJump24 => "ThumbJump24".to_owned(),
            RelocType::Branch => "Branch".to_owned(),
            RelocType::Jal => "Jal".to_owned(),
            RelocType::RvcBranch => "RvcBranch".to_owned(),
            RelocType::RvcJump => "RvcJump".to_owned(),
            RelocType::MovwAbsNc => "MovwAbsNc".to_owned(),
            RelocType::MovtAbs => "MovtAbs".to_owned(),
            RelocType::ThumbMovwAbsNc => "ThumbMovwAbsNc".to_owned(),
            RelocType::ThumbMovtAbs => "ThumbMovtAbs".to_owned(),
            RelocType::Prel31 => "Prel31".to_owned(),
            RelocType::Target1 => "Target1".to_owned(),
            RelocType::Target2 => "Target2".to_owned(),
            RelocType::AdrPrelLo21 => "AdrPrelLo21".to_owned(),
            RelocType::AdrPage21 => "AdrPage21".to_owned(),
            RelocType::AddAbsLo12 => "AddAbsLo12".to_owned(),
            RelocType::Ldst8Lo12 => "Ldst8Lo12".to_owned(),
            RelocType::Ldst16Lo12 => "Ldst16Lo12".to_owned(),
            RelocType::Ldst32Lo12 => "Ldst32Lo12".to_owned(),
            RelocType::Ldst64Lo12 => "Ldst64Lo12".to_owned(),
            RelocType::Ldst128Lo12 => "Ldst128Lo12".to_owned(),
            RelocType::Tstbr14 => "Tstbr14".to_owned(),
            RelocType::Condbr19 => "Condbr19".to_owned(),
            RelocType::AdrGotPage => "AdrGotPage".to_owned(),
            RelocType::Ld64GotLo12 => "Ld64GotLo12".to_owned(),
            RelocType::TlsGdPage21 => "TlsGdPage21".to_owned(),
            RelocType::TlsGdLo12 => "TlsGdLo12".to_owned(),
            RelocType::TlsLdPage21 => "TlsLdPage21".to_owned(),
            RelocType::TlsLdLo12 => "TlsLdLo12".to_owned(),
            RelocType::TlsLdDtprelHi12 => "TlsLdDtprelHi12".to_owned(),
            RelocType::TlsLdDtprelLo12 => "TlsLdDtprelLo12".to_owned(),
            RelocType::TlsIePage21 => "TlsIePage21".to_owned(),
            RelocType::TlsIeLo12 => "TlsIeLo12".to_owned(),
            RelocType::TlsLeTprelHi12 => "TlsLeTprelHi12".to_owned(),
            RelocType::TlsLeTprelLo12 => "TlsLeTprelLo12".to_owned(),
            RelocType::TlsDescPage21 => "TlsDescPage21".to_owned(),
            RelocType::TlsDescLd64Lo12 => "TlsDescLd64Lo12".to_owned(),
            RelocType::TlsDescAddLo12 => "TlsDescAddLo12".to_owned(),
            RelocType::TlsDescCall => "TlsDescCall".to_owned(),
            RelocType::GotHi20 => "GotHi20".to_owned(),
            RelocType::PcrelHi20 => "PcrelHi20".to_owned(),
            RelocType::PcrelLo12I => "PcrelLo12I".to_owned(),
            RelocType::PcrelLo12S => "PcrelLo12S".to_owned(),
            RelocType::Hi20 => "Hi20".to_owned(),
            RelocType::Lo12I => "Lo12I".to_owned(),
            RelocType::Lo12S => "Lo12S".to_owned(),
            RelocType::TlsGotHi20 => "TlsGotHi20".to_owned(),
            RelocType::TlsGdHi20 => "TlsGdHi20".to_owned(),
            RelocType::TprelHi20 => "TprelHi20".to_owned(),
            RelocType::TprelLo12I => "TprelLo12I".to_owned(),
            RelocType::TprelLo12S => "TprelLo12S".to_owned(),
            RelocType::TprelAdd => "TprelAdd".to_owned(),
            RelocType::Relax => "Relax".to_owned(),
            RelocType::Align => "Align".to_owned(),
            RelocType::Add8 => "Add8".to_owned(),
            RelocType::Add16 => "Add16".to_owned(),
            RelocType::Add32 => "Add32".to_owned(),
            RelocType::Add64 => "Add64".to_owned(),
            RelocType::Sub8 => "Sub8".to_owned(),
            RelocType::Sub16 => "Sub16".to_owned(),
            RelocType::Sub32 => "Sub32".to_owned(),
            RelocType::Sub64 => "Sub64".to_owned(),
            RelocType::Unknown(_) => "Unknown".to_owned(),
        }
    }

    #[rhai_fn(global, get = "field_0", pure)]
    pub fn get_field_0(value: &mut RelocType) -> Dynamic {
        match value {
            RelocType::None => Dynamic::UNIT,
            RelocType::Relative => Dynamic::UNIT,
            RelocType::Absolute => Dynamic::UNIT,
            RelocType::GlobDat => Dynamic::UNIT,
            RelocType::JumpSlot => Dynamic::UNIT,
            RelocType::Copy => Dynamic::UNIT,
            RelocType::IRelative => Dynamic::UNIT,
            RelocType::TlsDtpMod => Dynamic::UNIT,
            RelocType::TlsDtpOff => Dynamic::UNIT,
            RelocType::TlsTpOff => Dynamic::UNIT,
            RelocType::TlsDesc => Dynamic::UNIT,
            RelocType::PcRel32 => Dynamic::UNIT,
            RelocType::PcRel64 => Dynamic::UNIT,
            RelocType::Plt32 => Dynamic::UNIT,
            RelocType::Got32 => Dynamic::UNIT,
            RelocType::Got32X => Dynamic::UNIT,
            RelocType::GotOff => Dynamic::UNIT,
            RelocType::GotPc => Dynamic::UNIT,
            RelocType::GotPcRel => Dynamic::UNIT,
            RelocType::GotPcRelX => Dynamic::UNIT,
            RelocType::RexGotPcRelX => Dynamic::UNIT,
            RelocType::Abs32ZeroExt => Dynamic::UNIT,
            RelocType::Abs32SignExt => Dynamic::UNIT,
            RelocType::TlsGd => Dynamic::UNIT,
            RelocType::TlsLd => Dynamic::UNIT,
            RelocType::TlsLdo => Dynamic::UNIT,
            RelocType::TlsInitialExec => Dynamic::UNIT,
            RelocType::TlsLocalExec => Dynamic::UNIT,
            RelocType::Call => Dynamic::UNIT,
            RelocType::CallPlt => Dynamic::UNIT,
            RelocType::Jump24 => Dynamic::UNIT,
            RelocType::Jump26 => Dynamic::UNIT,
            RelocType::ThumbCall => Dynamic::UNIT,
            RelocType::ThumbJump24 => Dynamic::UNIT,
            RelocType::Branch => Dynamic::UNIT,
            RelocType::Jal => Dynamic::UNIT,
            RelocType::RvcBranch => Dynamic::UNIT,
            RelocType::RvcJump => Dynamic::UNIT,
            RelocType::MovwAbsNc => Dynamic::UNIT,
            RelocType::MovtAbs => Dynamic::UNIT,
            RelocType::ThumbMovwAbsNc => Dynamic::UNIT,
            RelocType::ThumbMovtAbs => Dynamic::UNIT,
            RelocType::Prel31 => Dynamic::UNIT,
            RelocType::Target1 => Dynamic::UNIT,
            RelocType::Target2 => Dynamic::UNIT,
            RelocType::AdrPrelLo21 => Dynamic::UNIT,
            RelocType::AdrPage21 => Dynamic::UNIT,
            RelocType::AddAbsLo12 => Dynamic::UNIT,
            RelocType::Ldst8Lo12 => Dynamic::UNIT,
            RelocType::Ldst16Lo12 => Dynamic::UNIT,
            RelocType::Ldst32Lo12 => Dynamic::UNIT,
            RelocType::Ldst64Lo12 => Dynamic::UNIT,
            RelocType::Ldst128Lo12 => Dynamic::UNIT,
            RelocType::Tstbr14 => Dynamic::UNIT,
            RelocType::Condbr19 => Dynamic::UNIT,
            RelocType::AdrGotPage => Dynamic::UNIT,
            RelocType::Ld64GotLo12 => Dynamic::UNIT,
            RelocType::TlsGdPage21 => Dynamic::UNIT,
            RelocType::TlsGdLo12 => Dynamic::UNIT,
            RelocType::TlsLdPage21 => Dynamic::UNIT,
            RelocType::TlsLdLo12 => Dynamic::UNIT,
            RelocType::TlsLdDtprelHi12 => Dynamic::UNIT,
            RelocType::TlsLdDtprelLo12 => Dynamic::UNIT,
            RelocType::TlsIePage21 => Dynamic::UNIT,
            RelocType::TlsIeLo12 => Dynamic::UNIT,
            RelocType::TlsLeTprelHi12 => Dynamic::UNIT,
            RelocType::TlsLeTprelLo12 => Dynamic::UNIT,
            RelocType::TlsDescPage21 => Dynamic::UNIT,
            RelocType::TlsDescLd64Lo12 => Dynamic::UNIT,
            RelocType::TlsDescAddLo12 => Dynamic::UNIT,
            RelocType::TlsDescCall => Dynamic::UNIT,
            RelocType::GotHi20 => Dynamic::UNIT,
            RelocType::PcrelHi20 => Dynamic::UNIT,
            RelocType::PcrelLo12I => Dynamic::UNIT,
            RelocType::PcrelLo12S => Dynamic::UNIT,
            RelocType::Hi20 => Dynamic::UNIT,
            RelocType::Lo12I => Dynamic::UNIT,
            RelocType::Lo12S => Dynamic::UNIT,
            RelocType::TlsGotHi20 => Dynamic::UNIT,
            RelocType::TlsGdHi20 => Dynamic::UNIT,
            RelocType::TprelHi20 => Dynamic::UNIT,
            RelocType::TprelLo12I => Dynamic::UNIT,
            RelocType::TprelLo12S => Dynamic::UNIT,
            RelocType::TprelAdd => Dynamic::UNIT,
            RelocType::Relax => Dynamic::UNIT,
            RelocType::Align => Dynamic::UNIT,
            RelocType::Add8 => Dynamic::UNIT,
            RelocType::Add16 => Dynamic::UNIT,
            RelocType::Add32 => Dynamic::UNIT,
            RelocType::Add64 => Dynamic::UNIT,
            RelocType::Sub8 => Dynamic::UNIT,
            RelocType::Sub16 => Dynamic::UNIT,
            RelocType::Sub32 => Dynamic::UNIT,
            RelocType::Sub64 => Dynamic::UNIT,
            RelocType::Unknown(raw) => Dynamic::from(*raw as i64),
        }
    }

    #[rhai_fn(global, set = "field_0", return_raw)]
    pub fn set_field_0(value: &mut RelocType, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            RelocType::None => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Relative => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Absolute => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::GlobDat => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::JumpSlot => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Copy => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::IRelative => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsDtpMod => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsDtpOff => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsTpOff => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsDesc => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::PcRel32 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::PcRel64 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Plt32 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Got32 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Got32X => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::GotOff => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::GotPc => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::GotPcRel => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::GotPcRelX => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::RexGotPcRelX => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Abs32ZeroExt => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Abs32SignExt => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsGd => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLd => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLdo => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsInitialExec => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLocalExec => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Call => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::CallPlt => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Jump24 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Jump26 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::ThumbCall => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::ThumbJump24 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Branch => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Jal => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::RvcBranch => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::RvcJump => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::MovwAbsNc => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::MovtAbs => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::ThumbMovwAbsNc => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::ThumbMovtAbs => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Prel31 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Target1 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Target2 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::AdrPrelLo21 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::AdrPage21 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::AddAbsLo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Ldst8Lo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Ldst16Lo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Ldst32Lo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Ldst64Lo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Ldst128Lo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Tstbr14 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Condbr19 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::AdrGotPage => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Ld64GotLo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsGdPage21 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsGdLo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLdPage21 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLdLo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLdDtprelHi12 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            RelocType::TlsLdDtprelLo12 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            RelocType::TlsIePage21 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsIeLo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLeTprelHi12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsLeTprelLo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsDescPage21 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsDescLd64Lo12 => {
                Err("field_0 is only writable on Unknown variants".into())
            }
            RelocType::TlsDescAddLo12 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsDescCall => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::GotHi20 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::PcrelHi20 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::PcrelLo12I => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::PcrelLo12S => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Hi20 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Lo12I => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Lo12S => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsGotHi20 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TlsGdHi20 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TprelHi20 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TprelLo12I => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TprelLo12S => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::TprelAdd => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Relax => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Align => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Add8 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Add16 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Add32 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Add64 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Sub8 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Sub16 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Sub32 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Sub64 => Err("field_0 is only writable on Unknown variants".into()),
            RelocType::Unknown(raw) => {
                *raw = new_value as u32;
                Ok(())
            }
        }
    }
}

#[allow(non_snake_case, non_upper_case_globals)]
#[export_module]
pub mod elfdyn_module {
    use crate::ElfDyn;

    pub const Null: ElfDyn = ElfDyn::Null;
    pub const Symbolic: ElfDyn = ElfDyn::Symbolic;
    pub const TextRel: ElfDyn = ElfDyn::TextRel;
    pub const BindNow: ElfDyn = ElfDyn::BindNow;

    pub fn Needed(value: i64) -> ElfDyn {
        ElfDyn::Needed(value as u64)
    }

    pub fn PltRelSz(value: i64) -> ElfDyn {
        ElfDyn::PltRelSz(value as u64)
    }

    pub fn PltGot(value: i64) -> ElfDyn {
        ElfDyn::PltGot(value as u64)
    }

    pub fn Hash(value: i64) -> ElfDyn {
        ElfDyn::Hash(value as u64)
    }

    pub fn StrTab(value: i64) -> ElfDyn {
        ElfDyn::StrTab(value as u64)
    }

    pub fn SymTab(value: i64) -> ElfDyn {
        ElfDyn::SymTab(value as u64)
    }

    pub fn Rela(value: i64) -> ElfDyn {
        ElfDyn::Rela(value as u64)
    }

    pub fn RelaSz(value: i64) -> ElfDyn {
        ElfDyn::RelaSz(value as u64)
    }

    pub fn RelaEnt(value: i64) -> ElfDyn {
        ElfDyn::RelaEnt(value as u64)
    }

    pub fn StrSz(value: i64) -> ElfDyn {
        ElfDyn::StrSz(value as u64)
    }

    pub fn SymEnt(value: i64) -> ElfDyn {
        ElfDyn::SymEnt(value as u64)
    }

    pub fn Init(value: i64) -> ElfDyn {
        ElfDyn::Init(value as u64)
    }

    pub fn Fini(value: i64) -> ElfDyn {
        ElfDyn::Fini(value as u64)
    }

    pub fn SoName(value: i64) -> ElfDyn {
        ElfDyn::SoName(value as u64)
    }

    pub fn RPath(value: i64) -> ElfDyn {
        ElfDyn::RPath(value as u64)
    }

    pub fn Rel(value: i64) -> ElfDyn {
        ElfDyn::Rel(value as u64)
    }

    pub fn RelSz(value: i64) -> ElfDyn {
        ElfDyn::RelSz(value as u64)
    }

    pub fn RelEnt(value: i64) -> ElfDyn {
        ElfDyn::RelEnt(value as u64)
    }

    pub fn PltRel(value: i64) -> ElfDyn {
        ElfDyn::PltRel(value as u64)
    }

    pub fn Debug(value: i64) -> ElfDyn {
        ElfDyn::Debug(value as u64)
    }

    pub fn JmpRel(value: i64) -> ElfDyn {
        ElfDyn::JmpRel(value as u64)
    }

    pub fn InitArray(value: i64) -> ElfDyn {
        ElfDyn::InitArray(value as u64)
    }

    pub fn FiniArray(value: i64) -> ElfDyn {
        ElfDyn::FiniArray(value as u64)
    }

    pub fn InitArraySz(value: i64) -> ElfDyn {
        ElfDyn::InitArraySz(value as u64)
    }

    pub fn FiniArraySz(value: i64) -> ElfDyn {
        ElfDyn::FiniArraySz(value as u64)
    }

    pub fn RunPath(value: i64) -> ElfDyn {
        ElfDyn::RunPath(value as u64)
    }

    pub fn Flags(value: i64) -> ElfDyn {
        ElfDyn::Flags(value as u64)
    }

    pub fn PreInitArray(value: i64) -> ElfDyn {
        ElfDyn::PreInitArray(value as u64)
    }

    pub fn PreInitArraySz(value: i64) -> ElfDyn {
        ElfDyn::PreInitArraySz(value as u64)
    }

    pub fn SunwAuxiliary(value: i64) -> ElfDyn {
        ElfDyn::SunwAuxiliary(value as u64)
    }

    pub fn SunwRtldInf(value: i64) -> ElfDyn {
        ElfDyn::SunwRtldInf(value as u64)
    }

    pub fn SunwFilter(value: i64) -> ElfDyn {
        ElfDyn::SunwFilter(value as u64)
    }

    pub fn SunwCap(value: i64) -> ElfDyn {
        ElfDyn::SunwCap(value as u64)
    }

    pub fn SunwSymTab(value: i64) -> ElfDyn {
        ElfDyn::SunwSymTab(value as u64)
    }

    pub fn SunwSymSz(value: i64) -> ElfDyn {
        ElfDyn::SunwSymSz(value as u64)
    }

    pub fn SunwSortEnt(value: i64) -> ElfDyn {
        ElfDyn::SunwSortEnt(value as u64)
    }

    pub fn SunwSymSort(value: i64) -> ElfDyn {
        ElfDyn::SunwSymSort(value as u64)
    }

    pub fn SunwSymSortSz(value: i64) -> ElfDyn {
        ElfDyn::SunwSymSortSz(value as u64)
    }

    pub fn SunwTlsSort(value: i64) -> ElfDyn {
        ElfDyn::SunwTlsSort(value as u64)
    }

    pub fn SunwTlsSortSz(value: i64) -> ElfDyn {
        ElfDyn::SunwTlsSortSz(value as u64)
    }

    pub fn SunwCapInfo(value: i64) -> ElfDyn {
        ElfDyn::SunwCapInfo(value as u64)
    }

    pub fn SunwStrPad(value: i64) -> ElfDyn {
        ElfDyn::SunwStrPad(value as u64)
    }

    pub fn SunwCapChain(value: i64) -> ElfDyn {
        ElfDyn::SunwCapChain(value as u64)
    }

    pub fn SunwLdMach(value: i64) -> ElfDyn {
        ElfDyn::SunwLdMach(value as u64)
    }

    pub fn SunwCapChainEnt(value: i64) -> ElfDyn {
        ElfDyn::SunwCapChainEnt(value as u64)
    }

    pub fn SunwCapChainSz(value: i64) -> ElfDyn {
        ElfDyn::SunwCapChainSz(value as u64)
    }

    pub fn Checksum(value: i64) -> ElfDyn {
        ElfDyn::Checksum(value as u64)
    }

    pub fn PltPadSz(value: i64) -> ElfDyn {
        ElfDyn::PltPadSz(value as u64)
    }

    pub fn MoveEnt(value: i64) -> ElfDyn {
        ElfDyn::MoveEnt(value as u64)
    }

    pub fn MoveSz(value: i64) -> ElfDyn {
        ElfDyn::MoveSz(value as u64)
    }

    pub fn PosFlag1(value: i64) -> ElfDyn {
        ElfDyn::PosFlag1(value as u64)
    }

    pub fn SymInSz(value: i64) -> ElfDyn {
        ElfDyn::SymInSz(value as u64)
    }

    pub fn SymInEnt(value: i64) -> ElfDyn {
        ElfDyn::SymInEnt(value as u64)
    }

    pub fn Config(value: i64) -> ElfDyn {
        ElfDyn::Config(value as u64)
    }

    pub fn DepAudit(value: i64) -> ElfDyn {
        ElfDyn::DepAudit(value as u64)
    }

    pub fn Audit(value: i64) -> ElfDyn {
        ElfDyn::Audit(value as u64)
    }

    pub fn PltPad(value: i64) -> ElfDyn {
        ElfDyn::PltPad(value as u64)
    }

    pub fn MoveTab(value: i64) -> ElfDyn {
        ElfDyn::MoveTab(value as u64)
    }

    pub fn SymInfo(value: i64) -> ElfDyn {
        ElfDyn::SymInfo(value as u64)
    }

    pub fn RelaCount(value: i64) -> ElfDyn {
        ElfDyn::RelaCount(value as u64)
    }

    pub fn RelCount(value: i64) -> ElfDyn {
        ElfDyn::RelCount(value as u64)
    }

    pub fn Flags1(value: i64) -> ElfDyn {
        ElfDyn::Flags1(value as u64)
    }

    pub fn VerDef(value: i64) -> ElfDyn {
        ElfDyn::VerDef(value as u64)
    }

    pub fn VerDefNum(value: i64) -> ElfDyn {
        ElfDyn::VerDefNum(value as u64)
    }

    pub fn VerNeed(value: i64) -> ElfDyn {
        ElfDyn::VerNeed(value as u64)
    }

    pub fn VerNeedNum(value: i64) -> ElfDyn {
        ElfDyn::VerNeedNum(value as u64)
    }

    pub fn SparcRegister(value: i64) -> ElfDyn {
        ElfDyn::SparcRegister(value as u64)
    }

    pub fn Auxiliary(value: i64) -> ElfDyn {
        ElfDyn::Auxiliary(value as u64)
    }

    pub fn Used(value: i64) -> ElfDyn {
        ElfDyn::Used(value as u64)
    }

    pub fn Filter(value: i64) -> ElfDyn {
        ElfDyn::Filter(value as u64)
    }

    pub fn Unknown(tag: i64, val: i64) -> ElfDyn {
        ElfDyn::Unknown {
            tag,
            val: val as u64,
        }
    }

    #[rhai_fn(global, get = "enum_type", pure)]
    pub fn get_type(value: &mut ElfDyn) -> String {
        match value {
            ElfDyn::Null => "Null".to_owned(),
            ElfDyn::Symbolic => "Symbolic".to_owned(),
            ElfDyn::TextRel => "TextRel".to_owned(),
            ElfDyn::BindNow => "BindNow".to_owned(),
            ElfDyn::Needed(_) => "Needed".to_owned(),
            ElfDyn::PltRelSz(_) => "PltRelSz".to_owned(),
            ElfDyn::PltGot(_) => "PltGot".to_owned(),
            ElfDyn::Hash(_) => "Hash".to_owned(),
            ElfDyn::StrTab(_) => "StrTab".to_owned(),
            ElfDyn::SymTab(_) => "SymTab".to_owned(),
            ElfDyn::Rela(_) => "Rela".to_owned(),
            ElfDyn::RelaSz(_) => "RelaSz".to_owned(),
            ElfDyn::RelaEnt(_) => "RelaEnt".to_owned(),
            ElfDyn::StrSz(_) => "StrSz".to_owned(),
            ElfDyn::SymEnt(_) => "SymEnt".to_owned(),
            ElfDyn::Init(_) => "Init".to_owned(),
            ElfDyn::Fini(_) => "Fini".to_owned(),
            ElfDyn::SoName(_) => "SoName".to_owned(),
            ElfDyn::RPath(_) => "RPath".to_owned(),
            ElfDyn::Rel(_) => "Rel".to_owned(),
            ElfDyn::RelSz(_) => "RelSz".to_owned(),
            ElfDyn::RelEnt(_) => "RelEnt".to_owned(),
            ElfDyn::PltRel(_) => "PltRel".to_owned(),
            ElfDyn::Debug(_) => "Debug".to_owned(),
            ElfDyn::JmpRel(_) => "JmpRel".to_owned(),
            ElfDyn::InitArray(_) => "InitArray".to_owned(),
            ElfDyn::FiniArray(_) => "FiniArray".to_owned(),
            ElfDyn::InitArraySz(_) => "InitArraySz".to_owned(),
            ElfDyn::FiniArraySz(_) => "FiniArraySz".to_owned(),
            ElfDyn::RunPath(_) => "RunPath".to_owned(),
            ElfDyn::Flags(_) => "Flags".to_owned(),
            ElfDyn::PreInitArray(_) => "PreInitArray".to_owned(),
            ElfDyn::PreInitArraySz(_) => "PreInitArraySz".to_owned(),
            ElfDyn::SunwAuxiliary(_) => "SunwAuxiliary".to_owned(),
            ElfDyn::SunwRtldInf(_) => "SunwRtldInf".to_owned(),
            ElfDyn::SunwFilter(_) => "SunwFilter".to_owned(),
            ElfDyn::SunwCap(_) => "SunwCap".to_owned(),
            ElfDyn::SunwSymTab(_) => "SunwSymTab".to_owned(),
            ElfDyn::SunwSymSz(_) => "SunwSymSz".to_owned(),
            ElfDyn::SunwSortEnt(_) => "SunwSortEnt".to_owned(),
            ElfDyn::SunwSymSort(_) => "SunwSymSort".to_owned(),
            ElfDyn::SunwSymSortSz(_) => "SunwSymSortSz".to_owned(),
            ElfDyn::SunwTlsSort(_) => "SunwTlsSort".to_owned(),
            ElfDyn::SunwTlsSortSz(_) => "SunwTlsSortSz".to_owned(),
            ElfDyn::SunwCapInfo(_) => "SunwCapInfo".to_owned(),
            ElfDyn::SunwStrPad(_) => "SunwStrPad".to_owned(),
            ElfDyn::SunwCapChain(_) => "SunwCapChain".to_owned(),
            ElfDyn::SunwLdMach(_) => "SunwLdMach".to_owned(),
            ElfDyn::SunwCapChainEnt(_) => "SunwCapChainEnt".to_owned(),
            ElfDyn::SunwCapChainSz(_) => "SunwCapChainSz".to_owned(),
            ElfDyn::Checksum(_) => "Checksum".to_owned(),
            ElfDyn::PltPadSz(_) => "PltPadSz".to_owned(),
            ElfDyn::MoveEnt(_) => "MoveEnt".to_owned(),
            ElfDyn::MoveSz(_) => "MoveSz".to_owned(),
            ElfDyn::PosFlag1(_) => "PosFlag1".to_owned(),
            ElfDyn::SymInSz(_) => "SymInSz".to_owned(),
            ElfDyn::SymInEnt(_) => "SymInEnt".to_owned(),
            ElfDyn::Config(_) => "Config".to_owned(),
            ElfDyn::DepAudit(_) => "DepAudit".to_owned(),
            ElfDyn::Audit(_) => "Audit".to_owned(),
            ElfDyn::PltPad(_) => "PltPad".to_owned(),
            ElfDyn::MoveTab(_) => "MoveTab".to_owned(),
            ElfDyn::SymInfo(_) => "SymInfo".to_owned(),
            ElfDyn::RelaCount(_) => "RelaCount".to_owned(),
            ElfDyn::RelCount(_) => "RelCount".to_owned(),
            ElfDyn::Flags1(_) => "Flags1".to_owned(),
            ElfDyn::VerDef(_) => "VerDef".to_owned(),
            ElfDyn::VerDefNum(_) => "VerDefNum".to_owned(),
            ElfDyn::VerNeed(_) => "VerNeed".to_owned(),
            ElfDyn::VerNeedNum(_) => "VerNeedNum".to_owned(),
            ElfDyn::SparcRegister(_) => "SparcRegister".to_owned(),
            ElfDyn::Auxiliary(_) => "Auxiliary".to_owned(),
            ElfDyn::Used(_) => "Used".to_owned(),
            ElfDyn::Filter(_) => "Filter".to_owned(),
            ElfDyn::Unknown { .. } => "Unknown".to_owned(),
        }
    }

    #[rhai_fn(global, get = "field_0", pure)]
    pub fn get_field_0(value: &mut ElfDyn) -> Dynamic {
        match value {
            ElfDyn::Null => Dynamic::UNIT,
            ElfDyn::Symbolic => Dynamic::UNIT,
            ElfDyn::TextRel => Dynamic::UNIT,
            ElfDyn::BindNow => Dynamic::UNIT,
            ElfDyn::Needed(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PltRelSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PltGot(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Hash(raw) => Dynamic::from(*raw as i64),
            ElfDyn::StrTab(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SymTab(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Rela(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RelaSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RelaEnt(raw) => Dynamic::from(*raw as i64),
            ElfDyn::StrSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SymEnt(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Init(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Fini(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SoName(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RPath(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Rel(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RelSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RelEnt(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PltRel(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Debug(raw) => Dynamic::from(*raw as i64),
            ElfDyn::JmpRel(raw) => Dynamic::from(*raw as i64),
            ElfDyn::InitArray(raw) => Dynamic::from(*raw as i64),
            ElfDyn::FiniArray(raw) => Dynamic::from(*raw as i64),
            ElfDyn::InitArraySz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::FiniArraySz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RunPath(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Flags(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PreInitArray(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PreInitArraySz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwAuxiliary(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwRtldInf(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwFilter(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwCap(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwSymTab(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwSymSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwSortEnt(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwSymSort(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwSymSortSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwTlsSort(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwTlsSortSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwCapInfo(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwStrPad(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwCapChain(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwLdMach(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwCapChainEnt(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SunwCapChainSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Checksum(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PltPadSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::MoveEnt(raw) => Dynamic::from(*raw as i64),
            ElfDyn::MoveSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PosFlag1(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SymInSz(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SymInEnt(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Config(raw) => Dynamic::from(*raw as i64),
            ElfDyn::DepAudit(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Audit(raw) => Dynamic::from(*raw as i64),
            ElfDyn::PltPad(raw) => Dynamic::from(*raw as i64),
            ElfDyn::MoveTab(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SymInfo(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RelaCount(raw) => Dynamic::from(*raw as i64),
            ElfDyn::RelCount(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Flags1(raw) => Dynamic::from(*raw as i64),
            ElfDyn::VerDef(raw) => Dynamic::from(*raw as i64),
            ElfDyn::VerDefNum(raw) => Dynamic::from(*raw as i64),
            ElfDyn::VerNeed(raw) => Dynamic::from(*raw as i64),
            ElfDyn::VerNeedNum(raw) => Dynamic::from(*raw as i64),
            ElfDyn::SparcRegister(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Auxiliary(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Used(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Filter(raw) => Dynamic::from(*raw as i64),
            ElfDyn::Unknown { tag, .. } => Dynamic::from(*tag),
        }
    }

    #[rhai_fn(global, set = "field_0", return_raw)]
    pub fn set_field_0(value: &mut ElfDyn, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            ElfDyn::Null => Err("field_0 is not available on unit variants".into()),
            ElfDyn::Symbolic => Err("field_0 is not available on unit variants".into()),
            ElfDyn::TextRel => Err("field_0 is not available on unit variants".into()),
            ElfDyn::BindNow => Err("field_0 is not available on unit variants".into()),
            ElfDyn::Needed(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PltRelSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PltGot(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Hash(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::StrTab(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SymTab(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Rela(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RelaSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RelaEnt(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::StrSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SymEnt(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Init(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Fini(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SoName(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RPath(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Rel(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RelSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RelEnt(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PltRel(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Debug(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::JmpRel(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::InitArray(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::FiniArray(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::InitArraySz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::FiniArraySz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RunPath(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Flags(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PreInitArray(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PreInitArraySz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwAuxiliary(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwRtldInf(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwFilter(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwCap(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwSymTab(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwSymSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwSortEnt(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwSymSort(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwSymSortSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwTlsSort(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwTlsSortSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwCapInfo(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwStrPad(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwCapChain(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwLdMach(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwCapChainEnt(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SunwCapChainSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Checksum(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PltPadSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::MoveEnt(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::MoveSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PosFlag1(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SymInSz(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SymInEnt(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Config(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::DepAudit(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Audit(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::PltPad(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::MoveTab(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SymInfo(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RelaCount(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::RelCount(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Flags1(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::VerDef(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::VerDefNum(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::VerNeed(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::VerNeedNum(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::SparcRegister(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Auxiliary(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Used(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Filter(raw) => {
                *raw = new_value as u64;
                Ok(())
            }
            ElfDyn::Unknown { tag, .. } => {
                *tag = new_value;
                Ok(())
            }
        }
    }

    #[rhai_fn(global, get = "field_1", pure)]
    pub fn get_field_1(value: &mut ElfDyn) -> Dynamic {
        match value {
            ElfDyn::Unknown { val, .. } => Dynamic::from(*val as i64),
            _ => Dynamic::UNIT,
        }
    }

    #[rhai_fn(global, set = "field_1", return_raw)]
    pub fn set_field_1(value: &mut ElfDyn, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            ElfDyn::Unknown { val, .. } => {
                *val = new_value as u64;
                Ok(())
            }
            _ => Err("field_1 is only available on Unknown variants".into()),
        }
    }

    #[rhai_fn(global, get = "tag", pure)]
    pub fn get_tag(value: &mut ElfDyn) -> Dynamic {
        match value {
            ElfDyn::Unknown { tag, .. } => Dynamic::from(*tag),
            _ => Dynamic::UNIT,
        }
    }

    #[rhai_fn(global, set = "tag", return_raw)]
    pub fn set_tag(value: &mut ElfDyn, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            ElfDyn::Unknown { tag, .. } => {
                *tag = new_value;
                Ok(())
            }
            _ => Err("tag is only available on Unknown variants".into()),
        }
    }

    #[rhai_fn(global, get = "val", pure)]
    pub fn get_val(value: &mut ElfDyn) -> Dynamic {
        match value {
            ElfDyn::Unknown { val, .. } => Dynamic::from(*val as i64),
            _ => Dynamic::UNIT,
        }
    }

    #[rhai_fn(global, set = "val", return_raw)]
    pub fn set_val(value: &mut ElfDyn, new_value: i64) -> Result<(), Box<EvalAltResult>> {
        match value {
            ElfDyn::Unknown { val, .. } => {
                *val = new_value as u64;
                Ok(())
            }
            _ => Err("val is only available on Unknown variants".into()),
        }
    }
}
