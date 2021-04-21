use crate::{ReadBuf, WriteBuf};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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
    pub fn parse(buf: &mut ReadBuf) -> Self {
        let magic = Magic::parse(buf);

        // reverse byte endian if necessary
        match magic {
            Magic::Cigam64 | Magic::Cigam | Magic::FatCigam => {
                buf.set_reverse_endian();
            }
            _ => {}
        }

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

    pub fn write(&self, buf: &mut WriteBuf) {
        self.magic.write(buf);
        self.cpu_type.write(buf);
        self.file_type.write(buf);
        buf.write_u32(self.n_cmds);
        buf.write_u32(self.size_of_cmds);
        self.flags.write(buf);

        if self.magic.is_64bit() {
            // write "reserved" field
            buf.write_u32(0);
        }
    }
}

/// An integer containing a value identifying this file as a Mach-O file.
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Magic {
    /// use if the file is intended for use on a 64bit CPU with the **same** endianness as the host
    /// computer.
    Magic64 = 0xfeedfacf,
    /// use if the file is intended for use on a 64bit CPU with the **reverse** endianness as the
    /// host computer.
    Cigam64 = 0xcffaedfe,
    /// use if the file is intended for use on a 32bit CPU with the **same** endianness as the host
    /// computer.
    Magic = 0xfeedface,
    /// use if the file is intended for use on a 32bit CPU with the **reverse** endianness as the
    /// host computer.
    Cigam = 0xcefaedfe,
    /// use if the file contains code for more than one architecture and is intended for use on a
    /// CPU with the **same** endianness as the host computer.
    FatMagic = 0xcafebabe,
    /// use if the file contains code for more than one architecture and is intended for use on a
    /// CPU with the **reverse** endianness as the host computer.
    FatCigam = 0xbebafeca,
}

impl Magic {
    pub fn is_64bit(&self) -> bool {
        matches!(self, Magic::Magic64 | Magic::Cigam64)
    }

    pub fn from_u32(n: u32) -> Option<Self> {
        FromPrimitive::from_u32(n)
    }

    pub fn to_u32(&self) -> u32 {
        *self as u32
    }

    pub fn parse(buf: &mut ReadBuf) -> Magic {
        assert!(buf.is_native_endian());

        let magic_n = buf.read_u32();
        Magic::from_u32(magic_n).expect("Invalid magic number")
    }

    pub fn write(&self, buf: &mut WriteBuf) {
        buf.write_u32(*self as u32);
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

    pub fn parse(buf: &mut ReadBuf) -> Self {
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

    pub fn write(&self, buf: &mut WriteBuf) {
        let (cpu_type_n, cpu_subtype_n) = self.to_i32_i32();
        buf.write_i32(cpu_type_n);
        buf.write_i32(cpu_subtype_n);
    }

    pub fn to_i32_i32(&self) -> (i32, i32) {
        match self {
            CpuType::X86(sub) => (CpuType::CPU_TYPE_X86, *sub as i32),
            CpuType::X86_64(sub) => (CpuType::CPU_TYPE_X86_64, *sub as i32),
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
    pub fn parse(buf: &mut ReadBuf) -> Self {
        let file_type_n = buf.read_u32();
        FileType::from_u32(file_type_n)
            .expect(format!("Unsupported file_type number {}", file_type_n).as_str())
    }

    pub fn write(&self, buf: &mut WriteBuf) {
        buf.write_u32(*self as u32);
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
    pub fn new() -> Flags {
        Flags { flags: Vec::new() }
    }

    pub fn push(&mut self, flag: Flag) {
        self.flags.push(flag);
    }

    pub fn parse(buf: &mut ReadBuf) -> Self {
        let flags_n = buf.read_u32();

        let mut flags = Flags::new();
        for i in 0..=31 {
            let flag_n = flags_n & (1 << i);
            if flag_n != 0 {
                let flag = Flag::from_u32(flag_n)
                    .expect(format!("Unsupported flag : {:#X}", flag_n).as_str());
                flags.push(flag);
            }
        }

        flags
    }

    pub fn write(&self, buf: &mut WriteBuf) {
        let mut flag_n = 0u32;

        for flag in self.flags.iter() {
            flag_n = flag_n | *flag as u32;
        }

        buf.write_u32(flag_n);
    }
}

impl fmt::Debug for Flags {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.flags.iter()).finish()
    }
}
