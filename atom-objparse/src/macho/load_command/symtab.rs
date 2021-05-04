use crate::reader::Reader;
use atom_macho::load_command::SymtabCommand;

pub fn parse_sym_tab(buf: &mut Reader) -> SymtabCommand {
    let cmd = buf.read_u32();
    if cmd != SymtabCommand::TYPE {
        panic!("Invalid cmd number");
    }

    let cmdsize = buf.read_u32();
    let symoff = buf.read_u32();
    let nsyms = buf.read_u32();
    let stroff = buf.read_u32();
    let strsize = buf.read_u32();

    SymtabCommand {
        cmd,
        cmdsize,
        symoff,
        nsyms,
        stroff,
        strsize,
    }
}
