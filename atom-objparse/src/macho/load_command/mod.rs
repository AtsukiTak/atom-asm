mod build_version;
mod dy_sym_tab;
mod segment64;
mod sym_tab;

use self::{
    build_version::parse_build_version, dy_sym_tab::parse_dy_sym_tab, segment64::parse_segment64,
    sym_tab::parse_sym_tab,
};
use crate::reader::Reader;
use atom_macho::load_command::{BuildVersion, DySymTab, LoadCommand, SegmentCommand64, SymTab};

pub fn parse_load_command(buf: &mut Reader) -> LoadCommand {
    use LoadCommand as LC;

    // peek read
    let cmd_type_n = buf.clone().read_u32();

    match cmd_type_n {
        SegmentCommand64::TYPE => {
            let (segment, sections) = parse_segment64(buf);
            LC::Segment64(segment, sections)
        }
        SymTab::CMD_TYPE => LC::SymTab(parse_sym_tab(buf)),
        BuildVersion::CMD_TYPE => LC::BuildVersion(parse_build_version(buf)),
        DySymTab::CMD_TYPE => LC::DySymTab(parse_dy_sym_tab(buf)),
        _ => panic!("Unsupported cmd_type 0x{:X}", cmd_type_n),
    }
}
