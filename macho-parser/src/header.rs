use crate::Magic;
use mach_object::{CPU_TYPE_POWERPC, CPU_TYPE_POWERPC64, CPU_TYPE_X86, CPU_TYPE_X86_64};
use num_derive::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub magic: Magic,
    pub cpu_type: CpuType,
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuType {
    X86_64 = CPU_TYPE_X86_64 as isize,
    PowerPC64 = CPU_TYPE_POWERPC64 as isize,
    X86 = CPU_TYPE_X86 as isize,
    PowerPC = CPU_TYPE_POWERPC as isize,
}
