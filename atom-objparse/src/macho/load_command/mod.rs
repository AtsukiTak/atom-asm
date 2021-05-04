mod build_version;
mod dysymtab;
mod segment64;
mod symtab;

use self::{
    build_version::parse_build_version, dysymtab::parse_dysymtab, segment64::parse_segment64,
    symtab::parse_sym_tab,
};
use crate::reader::Reader;
use atom_macho::load_command::{
    BuildVersionCommand, DysymtabCommand, LoadCommand, SegmentCommand64, SymtabCommand,
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
        SymtabCommand::TYPE => LC::Symtab(parse_sym_tab(buf)),
        BuildVersionCommand::TYPE => {
            let (build_ver, tool_ver) = parse_build_version(buf);
            LC::BuildVersion(build_ver, tool_ver)
        }
        DysymtabCommand::TYPE => LC::Dysymtab(parse_dysymtab(buf)),
        _ => panic!("Unsupported cmd_type 0x{:X}", cmd_type_n),
    }
}
