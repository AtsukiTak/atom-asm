#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymTab {
    pub cmd_size: u32,
    /// the byte offset from the start of the file to the location of the
    /// symbol table entries
    pub sym_off: u32,
    /// number of symbol table entries
    pub n_syms: u32,
    /// the byte offset from the start of the file to the location of the string table.
    pub str_off: u32,
    /// the size (in bytes) of the string table.
    pub str_size: u32,
}

impl SymTab {
    pub const CMD_TYPE: u32 = 0x2;

    #[rustfmt::skip]
    pub const CMD_SIZE: u32 =
        4       // cmd
        + 4     // cmdsize
        + 4     // symoff
        + 4     // nsyms
        + 4     // stroff
        + 4;
}
