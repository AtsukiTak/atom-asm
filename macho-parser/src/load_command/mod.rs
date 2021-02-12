mod segment_64;
mod sym_tab;

pub use self::segment_64::{Section64, SectionAttr, SectionAttrs, SectionType, Segment64};
pub use self::sym_tab::SymTab;

use crate::Buffer;
use mach_object as macho;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(Segment64),
    SymTab(SymTab),
}

impl LoadCommand {
    pub fn parse(buf: &mut Buffer) -> Self {
        let cmd_type_n = buf.read_u32();
        if cmd_type_n == macho::LC_SEGMENT_64 {
            LoadCommand::Segment64(Segment64::parse(buf))
        } else if cmd_type_n == macho::LC_SYMTAB {
            LoadCommand::SymTab(SymTab::parse(buf))
        } else {
            panic!("Unsupported cmd_type {}", cmd_type_n);
        }
    }
}
