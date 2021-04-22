pub mod build_version;
pub mod dy_sym_tab;
pub mod segment64;
pub mod sym_tab;

pub use self::build_version::BuildVersion;
pub use self::dy_sym_tab::DySymTab;
pub use self::segment64::Segment64;
pub use self::sym_tab::SymTab;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(Segment64),
    SymTab(SymTab),
    DySymTab(DySymTab),
    BuildVersion(BuildVersion),
}

impl LoadCommand {
    pub fn cmd(&self) -> u32 {
        use LoadCommand as LC;

        match self {
            LC::Segment64(_) => Segment64::CMD_TYPE,
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
