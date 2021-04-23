use crate::buffer::Buffer;
use atom_macho_types::load_command::SymTab;

pub fn parse_sym_tab(buf: &mut Buffer) -> SymTab {
    let cmd_type = buf.read_u32();
    if cmd_type != SymTab::CMD_TYPE {
        panic!("Invalid cmd number");
    }

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
