pub mod build_version;
pub mod dysymtab;
pub mod segment64;
pub mod symtab;

pub use self::{
    build_version::{BuildToolVersion, BuildVersionCommand},
    dysymtab::DysymtabCommand,
    segment64::{Section64, SegmentCommand64},
    symtab::SymtabCommand,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(SegmentCommand64, Vec<Section64>),
    Symtab(SymtabCommand),
    Dysymtab(DysymtabCommand),
    BuildVersion(BuildVersionCommand, Vec<BuildToolVersion>),
}

impl LoadCommand {
    pub fn cmd(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmd,
            LC::Symtab(cmd) => cmd.cmd,
            LC::Dysymtab(cmd) => cmd.cmd,
            LC::BuildVersion(cmd, _) => cmd.cmd,
        }
    }

    pub fn cmd_size(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmdsize,
            LC::Symtab(cmd) => cmd.cmdsize,
            LC::Dysymtab(cmd) => cmd.cmdsize,
            LC::BuildVersion(cmd, _) => cmd.cmdsize,
        }
    }
}
