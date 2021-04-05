pub mod build_version;
pub mod dy_sym_tab;
pub mod segment_64;
pub mod sym_tab;

use self::build_version::BuildVersion;
use self::dy_sym_tab::DySymTab;
use self::segment_64::Segment64;
use self::sym_tab::SymTab;

use crate::Buffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(Segment64),
    SymTab(SymTab),
    DySymTab(DySymTab),
    BuildVersion(BuildVersion),
}

impl LoadCommand {
    const LC_SEGMENT_64: u32 = 0x19;
    const LC_SYMTAB: u32 = 0x02;
    const LC_DYSYMTAB: u32 = 0x0B;
    const LC_BUILD_VERSION: u32 = 0x32;

    pub fn parse(buf: &mut Buffer) -> Self {
        use LoadCommand as LC;

        let cmd_type_n = buf.read_u32();
        match cmd_type_n {
            Self::LC_SEGMENT_64 => LC::Segment64(Segment64::parse(buf)),
            Self::LC_SYMTAB => LC::SymTab(SymTab::parse(buf)),
            Self::LC_BUILD_VERSION => LC::BuildVersion(BuildVersion::parse(buf)),
            Self::LC_DYSYMTAB => LC::DySymTab(DySymTab::parse(buf)),
            _ => panic!("Unsupported cmd_type 0x{:X}", cmd_type_n),
        }
    }

    pub fn cmd(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(_) => Self::LC_SEGMENT_64,
            LC::SymTab(_) => Self::LC_SYMTAB,
            LC::DySymTab(_) => Self::LC_DYSYMTAB,
            LC::BuildVersion(_) => Self::LC_BUILD_VERSION,
        }
    }

    pub fn cmd_size(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd) => cmd.cmd_size,
            LC::SymTab(cmd) => cmd.cmd_size,
            LC::DySymTab(cmd) => cmd.cmd_size,
            LC::BuildVersion(cmd) => cmd.cmd_size,
        }
    }
}
