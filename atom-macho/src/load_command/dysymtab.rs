/// This is the second set of the symbolic information which is used to support
/// the data structures for the dynamically link editor.
///
/// The original set of symbolic information in the symtab_command which contains
/// the symbol and string tables must also be present when this load command is
/// present.  When this load command is present the symbol table is organized
/// into three groups of symbols:
/// * local symbols (static and debugging symbols) - grouped by module
/// * defined external symbols - grouped by module (sorted by name if not lib)
/// * undefined external symbols (sorted by name if MH_BINDATLOAD is not set,
///   and in order the were seen by the static linker if MH_BINDATLOAD is set)
///
/// In this load command there are offsets and counts to each of the three groups of symbols.
///
/// This load command contains a the offsets and sizes of the following new
/// symbolic information tables:
/// * table of contents
/// * module table
/// * reference symbol table
/// * indirect symbol table
///
/// The first three tables above (the table of contents, module table and
/// reference symbol table) are only present if the file is a dynamically linked
/// shared library.  For executable and object modules, which are files
/// containing only one module, the information that would be in these three
/// tables is determined as follows:
/// * table of contents - the defined external symbols are sorted by name
/// * module table - the file contains only one module so everything in the
/// file is part of the module.
/// * reference symbol table - is the defined and undefined external symbols
///
/// For dynamically linked shared library files this load command also contains
/// offsets and sizes to the pool of relocation entries for all sections separated into two groups:
/// * external relocation entries
/// * local relocation entries
///
/// For executable and object modules the relocation entries continue to hang
/// off the section structures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DysymtabCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    /// index to local symbols
    pub ilocalsym: u32,
    /// number of local symbols
    pub nlocalsym: u32,
    /// index to externally defined symbols
    pub iextdefsym: u32,
    /// number of externally defined symbols
    pub nextdefsym: u32,
    /// index to undefined symbols
    pub iundefsym: u32,
    /// number of undefined symbols
    pub nundefsym: u32,
    /// file offset to table of contents
    pub tocoff: u32,
    /// number of entries in table of contents
    pub ntoc: u32,
    /// file offset to module table
    pub modtaboff: u32,
    /// number of module table entries
    pub nmodtab: u32,
    /// offset to referenced symbol table
    pub extrefsymoff: u32,
    /// number of referenced symbol table entries
    pub nextrefsyms: u32,
    /// file offset to the indirect symbol table
    pub indirectsymoff: u32,
    /// number of indirect symbol table entries
    pub nindirectsyms: u32,
    /// offset to external relocation entries
    pub extreloff: u32,
    /// number of external relocation entries
    pub nextrel: u32,
    /// offset to local relocation entries
    pub locreloff: u32,
    /// number of local relocation entries
    pub nlocrel: u32,
}

impl DysymtabCommand {
    pub const TYPE: u32 = 0xB;

    pub const SIZE: u32 = 0x50;
}
