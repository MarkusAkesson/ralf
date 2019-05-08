use crate::utils::{into_u32_le, into_u64_le};
use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Class {
    ELF32,
    ELF64,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Class: \t{}",
            match self {
                Class::ELF32 => "ELF32",
                Class::ELF64 => "ELF64",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Data: \t{}",
            match self {
                Endian::Little => "Little endian",
                Endian::Big => "Big endian",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OSABI {
    SystemV,
    HPUX,
    NetBSD,
    Linux,
    GNUHard,
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
    CloudABI,
    Unknown,
}

impl fmt::Display for OSABI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "OS/ABI: \t{}",
            match self {
                OSABI::SystemV => "System V",
                OSABI::HPUX => "HP-UX",
                OSABI::NetBSD => "NetBSD",
                OSABI::Linux => "Linux",
                OSABI::GNUHard => "GNU Hard",
                OSABI::Solaris => "Solaris",
                OSABI::AIX => "AIX",
                OSABI::IRIX => "IRIX",
                OSABI::FreeBSD => "FreeBSD",
                OSABI::Tru64 => "Tru64",
                OSABI::NovellModesto => "Novell Modesto",
                OSABI::OpenBSD => "OpenBSD",
                OSABI::OpenVMS => "OpenVMS",
                OSABI::NonStopKernel => "NonStop Kernel",
                OSABI::AROS => "AROS",
                OSABI::FenixOS => "Fenix OS",
                OSABI::CloudABI => "CloudABI",
                OSABI::Unknown => "Unknown",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TYPE {
    NONE,
    REL,
    EXEC,
    DYN,
    CORE,
    LOOS,
    HIOS,
    LOPROC,
    HIPROC,
}

impl fmt::Display for TYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type: \t{}",
            match self {
                TYPE::NONE => "NONE",
                TYPE::REL => "REL",
                TYPE::EXEC => "EXEC",
                TYPE::DYN => "DYN",
                TYPE::CORE => "CORE",
                TYPE::LOOS => "LOOS",
                TYPE::HIOS => "HIOS",
                TYPE::LOPROC => "LOPROC",
                TYPE::HIPROC => "HIPROC",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Machine {
    Unknown,
    SPARC,
    X86,
    MIPS,
    PowerPC,
    S390,
    ARM,
    SuperH,
    IA64,
    X8664,
    AArch64,
    RISCV,
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Machine: \t{}",
            match self {
                Machine::Unknown => "Unknown ISA",
                Machine::SPARC => "SPARC",
                Machine::X86 => "x86",
                Machine::MIPS => "MIPS",
                Machine::PowerPC => "POWERPC",
                Machine::S390 => "S390",
                Machine::ARM => "ARM",
                Machine::SuperH => "SuperH",
                Machine::IA64 => "IA-64",
                Machine::X8664 => "x86-64",
                Machine::AArch64 => "AARch64",
                Machine::RISCV => "RISC-V",
            }
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ElfHeader {
    pub class: Class,
    pub data: Endian,
    pub osabi: OSABI,
    pub abiversion: u8,
    pub e_type: TYPE,
    pub machine: Machine,
    pub entry: u64,
    pub phoff: u64,
    pub shoff: u64,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

const ELF_MAGIC: &[u8] = &[0x7f, 0x45, 0x4c, 0x46];

impl ElfHeader {
    pub fn new(header: &[u8]) -> Result<ElfHeader, &'static str> {
        if header[0..4] != *ELF_MAGIC {
            eprintln!("{:?}", &header[0..4]);
            return Err("No ELF header to parse");
        }

        let class = match header[0x04] {
            1 => Class::ELF32,
            2 => Class::ELF64,
            _ => return Err("Class not found"),
        };

        let data = match header[0x05] {
            1 => Endian::Little,
            2 => Endian::Big,
            _ => return Err("Endian not found"),
        };

        let osabi = match header[0x07] {
            0x00 => OSABI::SystemV,
            0x01 => OSABI::HPUX,
            0x02 => OSABI::NetBSD,
            0x03 => OSABI::Linux,
            0x04 => OSABI::GNUHard,
            0x06 => OSABI::Solaris,
            0x07 => OSABI::AIX,
            0x08 => OSABI::IRIX,
            0x09 => OSABI::FreeBSD,
            0x0A => OSABI::Tru64,
            0x0B => OSABI::NovellModesto,
            0x0C => OSABI::OpenBSD,
            0x0D => OSABI::OpenVMS,
            0x0E => OSABI::NonStopKernel,
            0x0F => OSABI::AROS,
            0x10 => OSABI::FenixOS,
            0x11 => OSABI::CloudABI,
            _ => OSABI::Unknown,
        };

        let abiversion = header[0x08];

        let e_type = header[0x10] as u16 | (header[0x11] as u16) << 8;
        let e_type = match e_type {
            0x00 => TYPE::NONE,
            0x01 => TYPE::REL,
            0x02 => TYPE::EXEC,
            0x03 => TYPE::DYN,
            0x04 => TYPE::CORE,
            0xfe00 => TYPE::LOOS,
            0xfeff => TYPE::HIOS,
            0xff00 => TYPE::LOPROC,
            0xffff => TYPE::HIPROC,
            _ => TYPE::NONE,
        };

        let machine = header[0x12] as u16 | (header[0x13] as u16) << 8;
        let machine = match machine {
            0x02 => Machine::SPARC,
            0x03 => Machine::X86,
            0x08 => Machine::MIPS,
            0x14 => Machine::PowerPC,
            0x16 => Machine::S390,
            0x28 => Machine::ARM,
            0x2a => Machine::SuperH,
            0x32 => Machine::IA64,
            0x3e => Machine::X8664,
            0xb7 => Machine::AArch64,
            0xf3 => Machine::RISCV,
            _ => Machine::Unknown,
        };

        let entry = if class == Class::ELF32 {
            into_u32_le(&header[0x18..0x18 + 4]) as u64
        } else {
            into_u64_le(&header[0x18..0x18 + 8])
        };

        let phoff = if class == Class::ELF32 {
            into_u32_le(&header[0x1c..0x1c + 4]) as u64
        } else {
            into_u64_le(&header[0x20..0x20 + 8])
        };

        let shoff = if class == Class::ELF32 {
            into_u32_le(&header[0x20..0x20 + 4]) as u64
        } else {
            into_u64_le(&header[0x28..0x28 + 8])
        };

        let ehsize = if class == Class::ELF32 {
            header[0x28] as u16 | (header[0x29] as u16) << 8
        } else {
            header[0x34] as u16 | (header[0x35] as u16) << 8
        };

        let phentsize = if class == Class::ELF32 {
            header[0x2a] as u16 | (header[0x2b] as u16) << 8
        } else {
            header[0x36] as u16 | (header[0x37] as u16) << 8
        };

        let phnum = if class == Class::ELF32 {
            header[0x2c] as u16 | (header[0x2d] as u16) << 8
        } else {
            header[0x38] as u16 | (header[0x39] as u16) << 8
        };

        let shentsize = if class == Class::ELF32 {
            header[0x2e] as u16 | (header[0x2f] as u16) << 8
        } else {
            header[0x3a] as u16 | (header[0x3b] as u16) << 8
        };

        let shnum = if class == Class::ELF32 {
            header[0x30] as u16 | (header[0x31] as u16) << 8
        } else {
            header[0x3c] as u16 | (header[0x3d] as u16) << 8
        };

        let shstrndx = if class == Class::ELF32 {
            header[0x32] as u16 | (header[0x33] as u16) << 8
        } else {
            header[0x3e] as u16 | (header[0x3f] as u16) << 8
        };

        Ok(ElfHeader {
            class,
            data,
            osabi,
            abiversion,
            e_type,
            machine,
            entry,
            phoff,
            shoff,
            ehsize,
            phentsize,
            phnum,
            shentsize,
            shnum,
            shstrndx,
        })
    }
}

impl fmt::Display for ElfHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Elf Header:")?;
        writeln!(f, "  {}", self.class)?;
        writeln!(f, "  {}", self.data)?;
        writeln!(f, "  {}", self.osabi)?;
        writeln!(f, "  ABI Version:\t{}", self.abiversion)?;
        writeln!(f, "  {}", self.e_type)?;
        writeln!(f, "  {}", self.machine)?;
        writeln!(f, "  Entry point address:\t0x{:x}", self.entry)?;
        writeln!(
            f,
            "  Start of program headers:\t{} (bytes into file)",
            self.phoff
        )?;
        writeln!(
            f,
            "  Start of section headers:\t{} (bytes into file)",
            self.shoff
        )?;
        writeln!(f, "  Size of this header:\t{} (bytes)", self.ehsize)?;
        writeln!(f, "  Size of program headers:\t{} (bytes)", self.phentsize)?;
        writeln!(f, "  Number of program headers:\t{}", self.phnum)?;
        writeln!(f, "  Size of section headers:\t{} (bytes)", self.shentsize)?;
        writeln!(f, "  Number of section headers:\t{}", self.shnum)?;
        writeln!(f, "  Section header string table index:\t{}", self.shstrndx)?;
        Ok(())
    }
}
