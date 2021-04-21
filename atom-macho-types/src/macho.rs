use crate::{header::Header64, LoadCommand};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachO {
    pub header: Header64,
    pub load_commands: Vec<LoadCommand>,
}
