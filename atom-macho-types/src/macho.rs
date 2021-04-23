use crate::{header::Header64, load_command::LoadCommand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachO {
    pub header: Header64,
    pub load_commands: Vec<LoadCommand>,
}
