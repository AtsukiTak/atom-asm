use atom_macho::header::{CpuSubTypeX86_64, CpuType, FileType, Flags, Header64, Magic};

pub trait Header64Configure {
    fn new_x86_64() -> Self;
}

impl Header64Configure for Header64 {
    fn new_x86_64() -> Header64 {
        Header64 {
            magic: Magic::Magic64,
            cpu_type: CpuType::X86_64(CpuSubTypeX86_64::All),
            file_type: FileType::Object,
            n_cmds: 0,
            size_of_cmds: 0,
            flags: Flags::new(),
            reserved: 0,
        }
    }
}
