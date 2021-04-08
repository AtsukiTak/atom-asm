use crate::{Buffer, Magic};
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
        let magic = Magic::parse(buf);
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

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86 {
    All = 0x3,
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86_64 {
    All = 0x3,
}

impl CpuType {
    const CPU_ARCH_ABI64: i32 = 0x01000000;
    const CPU_TYPE_X86: i32 = 0x7;
    const CPU_TYPE_X86_64: i32 = Self::CPU_TYPE_X86 | Self::CPU_ARCH_ABI64;

    pub fn parse(buf: &mut Buffer) -> Self {
        let cpu_type_n = buf.read_i32();
        let cpu_subtype_n = buf.read_i32();
        Self::from_i32_i32(cpu_type_n, cpu_subtype_n)
    }

    pub fn from_i32_i32(cpu_type_n: i32, cpu_subtype_n: i32) -> Self {
        // x86
        if cpu_type_n == Self::CPU_TYPE_X86 {
            let cpu_subtype = CpuSubTypeX86::from_i32(cpu_subtype_n).unwrap_or_else(|| {
                panic!("Unsupported cpu_subtype {} of x86 cpu_type", cpu_subtype_n)
            });
            CpuType::X86(cpu_subtype)
        // x86_64
        } else if cpu_type_n == Self::CPU_TYPE_X86_64 {
            let cpu_subtype = CpuSubTypeX86_64::from_i32(cpu_subtype_n).unwrap_or_else(|| {
                panic!(
                    "Unsupported cpu_subtype {} of x86_64 cpu_type",
                    cpu_subtype_n
                )
            });
            CpuType::X86_64(cpu_subtype)
        } else {
            panic!("Unsupported cpu_type {}", cpu_type_n);
        }
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
/// Declared in /usr/include/mach-o/loader.h
pub enum FileType {
    Object = 0x1,
    Execute = 0x2,
    FVMLib = 0x3,
    Core = 0x4,
    Preload = 0x5,
    Dylib = 0x6,
    Dylinker = 0x7,
    Bundle = 0x8,
    Dsym = 0xA,
}

impl FileType {
    pub fn parse(buf: &mut Buffer) -> Self {
        let file_type_n = buf.read_u32();
        FileType::from_u32(file_type_n)
            .expect(format!("Unsupported file_type number {}", file_type_n).as_str())
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Flag {
    NoUndefs                = 0x000001,
    IncrLink                = 0x000002,
    DyldLink                = 0x000004,
    BindAtLoad              = 0x000008,
    PreBound                = 0x000010,
    SplitSegs               = 0x000020,
    TwoLevel                = 0x000080,
    ForceFlat               = 0x000100,
    NoMultiDefs             = 0x000200,
    NoFixPreBinding         = 0x000400,
    PreBindable             = 0x000800,
    AllModsBound            = 0x001000,
    SubsectionsViaSymbols   = 0x002000,
    Canonical               = 0x004000,
    Pie                     = 0x200000,
    HasTlvDescriptors       = 0x800000,
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
                let flag = Flag::from_u32(flag_n)
                    .expect(format!("Unsupported flag : {:#X}", flag_n).as_str());
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
