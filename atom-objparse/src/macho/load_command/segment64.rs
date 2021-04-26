use crate::reader::Reader;
use atom_macho::load_command::segment64::{Section64, SectionAttrs, SectionType, Segment64};

pub fn parse_segment64(buf: &mut Reader) -> Segment64 {
    let start_pos = buf.pos();

    let cmd_type = buf.read_u32();
    if cmd_type != Segment64::CMD_TYPE {
        panic!("Invalid cmd number");
    }

    let cmd_size = buf.read_u32();
    let seg_name = buf.read_fixed_size_string(16);
    let vm_addr = buf.read_u64();
    let vm_size = buf.read_u64();
    let file_off = buf.read_u64();
    let file_size = buf.read_u64();
    let max_prot = buf.read_i32();
    let init_prot = buf.read_i32();
    let nsects = buf.read_u32();
    let flags = buf.read_u32();

    let mut sections = Vec::with_capacity(nsects as usize);
    for _ in 0..nsects {
        sections.push(parse_section64(buf));
    }

    let command = Segment64 {
        cmd_size,
        seg_name,
        vm_addr,
        vm_size,
        file_off,
        file_size,
        max_prot,
        init_prot,
        flags,
        sections,
    };

    // バイト境界は8に揃えられているので
    // その分をskipする
    let parsed = buf.pos() - start_pos;
    let alignment = (8 - (parsed % 8)) % 8;
    buf.skip(alignment);

    command
}

fn parse_section64(buf: &mut Reader) -> Section64 {
    let sect_name = buf.read_fixed_size_string(16);
    let seg_name = buf.read_fixed_size_string(16);
    let addr = buf.read_u64();
    let size = buf.read_u64();
    let offset = buf.read_u32();
    let align = buf.read_u32();
    let reloff = buf.read_u32();
    let nreloc = buf.read_u32();

    let flags_n = buf.read_u32();
    let sect_type = SectionType::from_u32(flags_n & 0x000000ff);
    let sect_attrs = SectionAttrs::from_u32(flags_n & 0xffffff00);

    let reserved_1 = buf.read_u32();
    let reserved_2 = buf.read_u32();
    let reserved_3 = buf.read_u32();

    Section64 {
        sect_name,
        seg_name,
        addr,
        size,
        offset,
        align,
        reloff,
        nreloc,
        flags: (sect_attrs, sect_type),
        reserved_1,
        reserved_2,
        reserved_3,
    }
}
