use crate::Buffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymTab {
    cmd_size: u32,
    sym_off: u32,
    n_syms: u32,
    str_off: u32,
    str_size: u32,
}

impl SymTab {
    pub fn parse(buf: &mut Buffer) -> Self {
        let cmd_size = buf.read_u32();
        let sym_off = buf.read_u32();
        let n_syms = buf.read_u32();
        let str_off = buf.read_u32();
        let str_size = buf.read_u32();

        SymTab {
            cmd_size,
            sym_off,
            n_syms,
            str_off,
            str_size,
        }
    }
}
