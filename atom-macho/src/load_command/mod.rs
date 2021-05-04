pub mod build_version;
pub mod dy_sym_tab;
pub mod segment64;
pub mod symtab;

pub use self::{
    build_version::{BuildToolVersion, BuildVersionCommand},
    dy_sym_tab::DySymtabCommand,
    segment64::{Section64, SegmentCommand64},
    symtab::SymtabCommand,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(SegmentCommand64, Vec<Section64>),
    SymtabCommand(SymtabCommand),
    DySymtabCommand(DySymtabCommand),
    BuildVersion(BuildVersionCommand, Vec<BuildToolVersion>),
}

impl LoadCommand {
    pub fn cmd(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmd,
            LC::SymtabCommand(cmd) => cmd.cmd,
            LC::DySymtabCommand(_) => DySymtabCommand::CMD_TYPE,
            LC::BuildVersion(cmd, _) => cmd.cmd,
        }
    }

    pub fn cmd_size(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(cmd, _) => cmd.cmdsize,
            LC::SymtabCommand(cmd) => cmd.cmdsize,
            LC::DySymtabCommand(cmd) => cmd.cmd_size,
            LC::BuildVersion(cmd, _) => cmd.cmdsize,
        }
    }
}
