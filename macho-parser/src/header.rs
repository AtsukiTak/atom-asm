use crate::Magic;
use mach_object as macho;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub magic: Magic,
    pub cpu_type: CpuType,
    pub file_type: FileType,
    pub n_cmds: u32,
    pub size_of_cmds: u32,
    pub flags: Vec<Flag>,
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
            // x86
            n if n == macho::CPU_TYPE_X86 => match cpu_subtype_n {
                n if n == macho::CPU_SUBTYPE_X86_ALL => CpuType::X86(CpuSubTypeX86::All),
                _ => panic!("Unsupported cpu_subtype : {}", cpu_subtype_n),
            },

            // x86_64
            n if n == macho::CPU_TYPE_X86_64 => match cpu_subtype_n {
                n if n == macho::CPU_SUBTYPE_X86_64_ALL => CpuType::X86_64(CpuSubTypeX86_64::All),
                _ => panic!("Unsupported cpu_subtype : {}", cpu_subtype_n),
            },

            // others
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
}

impl Flag {
    pub fn vec_from_u32(flags: u32) -> Vec<Flag> {
        let mut vec = Vec::new();
        for i in 0..=31 {
            if flags & (1 << i) != 0 {
                let flag = Flag::from_u32(1 << i).expect("Invalid flag");
                vec.push(flag);
            }
        }
        vec
    }
}
