use crate::{Buffer, Magic};
use mach_object as macho;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub magic: Magic,
    pub cpu_type: CpuType,
    pub file_type: FileType,
    pub n_cmds: u32,
    pub size_of_cmds: u32,
    pub flags: Flags,
}

impl Header {
    pub fn parse(buf: &mut Buffer) -> Self {
        let magic = *buf.magic();
        let cpu_type = CpuType::parse(buf);
        let file_type = FileType::parse(buf);
        let n_cmds = buf.read_u32();
        let size_of_cmds = buf.read_u32();
        let flags = Flags::parse(buf);

        if magic.is_64bit() {
            // read "reserved" field
            let _ = buf.read_u32();
        }

        Header {
            magic,
            cpu_type,
            file_type,
            n_cmds,
            size_of_cmds,
            flags,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuType {
    X86(CpuSubTypeX86),
    X86_64(CpuSubTypeX86_64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86 {
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86_64 {
    All,
}

impl CpuType {
    pub fn parse(buf: &mut Buffer) -> Self {
        let cpu_type_n = buf.read_i32();
        let cpu_subtype_n = buf.read_i32();
        Self::from_i32_i32(cpu_type_n, cpu_subtype_n)
    }

    pub fn from_i32_i32(cpu_type_n: i32, cpu_subtype_n: i32) -> Self {
        // x86
        if cpu_type_n == macho::CPU_TYPE_X86 {
            if cpu_subtype_n == macho::CPU_SUBTYPE_X86_ALL {
                CpuType::X86(CpuSubTypeX86::All)
            } else {
                panic!("Unsupported cpu_subtype {} of x86 cpu_type", cpu_subtype_n);
            }
        // x86_64
        } else if cpu_type_n == macho::CPU_TYPE_X86_64 {
            if cpu_subtype_n == macho::CPU_SUBTYPE_X86_64_ALL {
                CpuType::X86_64(CpuSubTypeX86_64::All)
            } else {
                panic!(
                    "Unsupported cpu_subtype {} of x86_64 cpu_type",
                    cpu_subtype_n
                );
            }
        } else {
            panic!("Unsupported cpu_type {}", cpu_type_n);
        }
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Object = macho::MH_OBJECT as isize,
    Execute = macho::MH_EXECUTE as isize,
    Bundle = macho::MH_BUNDLE as isize,
    Dylib = macho::MH_DYLIB as isize,
    Preload = macho::MH_PRELOAD as isize,
    Core = macho::MH_CORE as isize,
    Dylinker = macho::MH_DYLINKER as isize,
    Dsym = macho::MH_DSYM as isize,
}

impl FileType {
    pub fn parse(buf: &mut Buffer) -> Self {
        let file_type_n = buf.read_u32();
        FileType::from_u32(file_type_n)
            .expect(format!("Invalid file_type number {}", file_type_n).as_str())
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    NoUndefs = macho::MH_NOUNDEFS as isize,
    IncrLink = macho::MH_INCRLINK as isize,
    DyldLink = macho::MH_DYLDLINK as isize,
    TwoLevel = macho::MH_TWOLEVEL as isize,
    BindAtLoad = macho::MH_BINDATLOAD as isize,
    PreBound = macho::MH_PREBOUND as isize,
    PreBindable = macho::MH_PREBINDABLE as isize,
    NoFixPreBinding = macho::MH_NOFIXPREBINDING as isize,
    AllModsBound = macho::MH_ALLMODSBOUND as isize,
    Canonical = macho::MH_CANONICAL as isize,
    SplitSegs = macho::MH_SPLIT_SEGS as isize,
    ForceFlat = macho::MH_FORCE_FLAT as isize,
    SubsectionsViaSymbols = macho::MH_SUBSECTIONS_VIA_SYMBOLS as isize,
    NoMultiDefs = macho::MH_NOMULTIDEFS as isize,
    Pie = macho::MH_PIE as isize,
    HasTlvDescriptors = macho::MH_HAS_TLV_DESCRIPTORS as isize,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Flags {
    flags: Vec<Flag>,
}

impl Flags {
    pub fn parse(buf: &mut Buffer) -> Self {
        let flags_n = buf.read_u32();

        let mut flags = Vec::new();
        for i in 0..=31 {
            let flag_n = flags_n & (1 << i);
            if flag_n != 0 {
                let flag =
                    Flag::from_u32(flag_n).expect(format!("Invalid flag : {:#X}", flag_n).as_str());
                flags.push(flag);
            }
        }

        Flags { flags }
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.flags.iter()).finish()
    }
}
