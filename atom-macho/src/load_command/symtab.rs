#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymtabCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    /// the byte offset from the start of the file to the location of the
    /// symbol table entries
    pub symoff: u32,
    /// number of symbol table entries
    pub nsyms: u32,
    /// the byte offset from the start of the file to the location of the string table.
    pub stroff: u32,
    /// the size (in bytes) of the string table.
    pub strsize: u32,
}

impl SymtabCommand {
    pub const TYPE: u32 = 0x2;

    #[rustfmt::skip]
    pub const SIZE: u32 =
        4       // cmd
        + 4     // cmdsize
        + 4     // symoff
        + 4     // nsyms
        + 4     // stroff
        + 4; // strsize
}
