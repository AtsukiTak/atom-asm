mod build_version;
mod dy_sym_tab;
mod segment64;
mod symtab;

use self::{
    build_version::parse_build_version, dy_sym_tab::parse_dy_sym_tab, segment64::parse_segment64,
    symtab::parse_sym_tab,
};
use crate::reader::Reader;
use atom_macho::load_command::{
    BuildVersion, DySymtabCommand, LoadCommand, SegmentCommand64, SymtabCommand,
};

pub fn parse_load_command(buf: &mut Reader) -> LoadCommand {
    use LoadCommand as LC;

    // peek read
    let cmd_type_n = buf.clone().read_u32();

    match cmd_type_n {
        SegmentCommand64::TYPE => {
            let (segment, sections) = parse_segment64(buf);
            LC::Segment64(segment, sections)
        }
        SymtabCommand::TYPE => LC::SymtabCommand(parse_sym_tab(buf)),
        BuildVersion::CMD_TYPE => LC::BuildVersion(parse_build_version(buf)),
        DySymtabCommand::CMD_TYPE => LC::DySymtabCommand(parse_dy_sym_tab(buf)),
        _ => panic!("Unsupported cmd_type 0x{:X}", cmd_type_n),
    }
}
