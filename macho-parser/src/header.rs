use crate::Magic;
use mach_object as macho;
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub magic: Magic,
    pub cpu_type: CpuType,
    pub file_type: FileType,
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
    pub fn from_i32_i32(cpu_type_n: i32, cpu_subtype_n: i32) -> Self {
        match cpu_type_n {
            n if n == macho::CPU_TYPE_X86 => match cpu_subtype_n {
                n if n == macho::CPU_SUBTYPE_X86_ALL => CpuType::X86(CpuSubTypeX86::All),
                _ => panic!("Unsupported cpu_subtype : {}", cpu_subtype_n),
            },
            n if n == macho::CPU_TYPE_X86_64 => match cpu_subtype_n {
                n if n == macho::CPU_SUBTYPE_X86_64_ALL => CpuType::X86_64(CpuSubTypeX86_64::All),
                _ => panic!("Unsupported cpu_subtype : {}", cpu_subtype_n),
            },
            _ => panic!("Unsupported cpu_type : {}", cpu_type_n),
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
