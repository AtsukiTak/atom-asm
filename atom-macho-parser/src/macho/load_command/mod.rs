mod segment64;

use crate::buffer::Buffer;
use atom_macho_types::load_command::{BuildVersion, DySymTab, LoadCommand, Segment64, SymTab};

pub fn parse_load_command(buf: &mut Buffer) -> LoadCommand {
    use LoadCommand as LC;

    // peek read
    let cmd_type_n = buf.read_u32();
    buf.set_pos(buf.pos() - 4);

    match cmd_type_n {
        Segment64::CMD_TYPE => LC::Segment64(segment64::parse_segment64(buf)),
        // SymTab::COMMAND => LC::SymTab(SymTab::parse(buf)),
        // BuildVersion::COMMAND => LC::BuildVersion(BuildVersion::parse(buf)),
        // DySymTab::COMMAND => LC::DySymTab(DySymTab::parse(buf)),
        _ => panic!("Unsupported cmd_type 0x{:X}", cmd_type_n),
    }
}
