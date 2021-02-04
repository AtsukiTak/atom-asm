use crate::Magic;
use mach_object::{CPU_SUBTYPE_X86_64_ALL, CPU_SUBTYPE_X86_ALL, CPU_TYPE_X86, CPU_TYPE_X86_64};
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub magic: Magic,
    pub cpu_type: CpuType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuType {
    X86(CpuSubTypeX86),
    X86_64(CpuSubTypeX86_64),
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86 {
    All,
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuSubTypeX86_64 {
    All,
}

impl CpuType {
    pub fn from_i32_i32(cpu_type_n: i32, cpu_subtype_n: i32) -> Self {
        match cpu_type_n {
            n if n == CPU_TYPE_X86 => match cpu_subtype_n {
                n if n == CPU_SUBTYPE_X86_ALL => CpuType::X86(CpuSubTypeX86::All),
                _ => panic!("Unsupported cpu_subtype : {}", cpu_subtype_n),
            },
            n if n == CPU_TYPE_X86_64 => match cpu_subtype_n {
                n if n == CPU_SUBTYPE_X86_64_ALL => CpuType::X86_64(CpuSubTypeX86_64::All),
                _ => panic!("Unsupported cpu_subtype : {}", cpu_subtype_n),
            },
            _ => panic!("Unsupported cpu_type : {}", cpu_type_n),
        }
    }
}
