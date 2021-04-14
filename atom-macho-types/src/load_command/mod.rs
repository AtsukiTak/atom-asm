pub mod build_version;
pub mod dy_sym_tab;
pub mod segment_64;
pub mod sym_tab;

use self::build_version::BuildVersion;
use self::dy_sym_tab::DySymTab;
use self::segment_64::Segment64;
use self::sym_tab::SymTab;

use crate::{ReadBuf, WriteBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(Segment64),
    SymTab(SymTab),
    DySymTab(DySymTab),
    BuildVersion(BuildVersion),
}

impl LoadCommand {
    pub fn parse(buf: &mut ReadBuf) -> Self {
        use LoadCommand as LC;

        // peek read
        let cmd_type_n = buf.read_u32();
        buf.set_pos(buf.pos() - 4);

        match cmd_type_n {
            Segment64::COMMAND => LC::Segment64(Segment64::parse(buf)),
            SymTab::COMMAND => LC::SymTab(SymTab::parse(buf)),
            BuildVersion::COMMAND => LC::BuildVersion(BuildVersion::parse(buf)),
            DySymTab::COMMAND => LC::DySymTab(DySymTab::parse(buf)),
            _ => panic!("Unsupported cmd_type 0x{:X}", cmd_type_n),
        }
    }

    pub fn write(&self, buf: &WriteBuf) {
        todo!()
    }

    pub fn cmd(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(_) => Segment64::COMMAND,
            LC::SymTab(_) => SymTab::COMMAND,
            LC::DySymTab(_) => DySymTab::COMMAND,
            LC::BuildVersion(_) => BuildVersion::COMMAND,
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
