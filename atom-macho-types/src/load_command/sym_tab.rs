#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymTab {
    pub cmd_size: u32,
    pub sym_off: u32,
    pub n_syms: u32,
    /// An integer containing the byte offset from the start of the image to the location of the string table.
    pub str_off: u32,
    /// the size (in bytes) of the string table.
    pub str_size: u32,
}

impl SymTab {
    pub const CMD_TYPE: u32 = 0x2;
}
