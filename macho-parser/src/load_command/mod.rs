mod build_version;
mod segment_64;
mod sym_tab;

pub use self::build_version::{BuildToolVersion, BuildVersion};
pub use self::segment_64::{Section64, SectionAttr, SectionAttrs, SectionType, Segment64};
pub use self::sym_tab::SymTab;

use crate::Buffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(Segment64),
    SymTab(SymTab),
    BuildVersion(BuildVersion),
}

impl LoadCommand {
    const LC_SEGMENT_64: u32 = 0x19;
    const LC_SYMTAB: u32 = 0x02;
    const LC_BUILD_VERSION: u32 = 0x32;

    pub fn parse(buf: &mut Buffer) -> Self {
        use LoadCommand as LC;

        let cmd_type_n = buf.read_u32();
        match cmd_type_n {
            Self::LC_SEGMENT_64 => LC::Segment64(Segment64::parse(buf)),
            Self::LC_SYMTAB => LC::SymTab(SymTab::parse(buf)),
            Self::LC_BUILD_VERSION => LC::BuildVersion(BuildVersion::parse(buf)),
            _ => panic!("Unsupported cmd_type 0x{:X}", cmd_type_n),
        }
    }
}
