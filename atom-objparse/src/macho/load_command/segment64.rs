use crate::reader::Reader;
use atom_macho::load_command::segment64::{Section64, SectionAttrs, SectionType, SegmentCommand64};

pub fn parse_segment64(buf: &mut Reader) -> (SegmentCommand64, Vec<Section64>) {
    let start_pos = buf.pos();

    let cmd_type = buf.read_u32();
    if cmd_type != SegmentCommand64::TYPE {
        panic!("Invalid cmd number");
    }

    let cmdsize = buf.read_u32();
    let segname = buf.read_fixed_size_string(16);
    let vmaddr = buf.read_u64();
    let vmsize = buf.read_u64();
    let fileoff = buf.read_u64();
    let filesize = buf.read_u64();
    let maxprot = buf.read_i32();
    let initprot = buf.read_i32();
    let nsects = buf.read_u32();
    let flags = buf.read_u32();

    let segment_cmd = SegmentCommand64 {
        cmd: SegmentCommand64::TYPE,
        cmdsize,
        segname,
        vmaddr,
        vmsize,
        fileoff,
        filesize,
        maxprot,
        initprot,
        nsects,
        flags,
    };

    let mut section_cmds = Vec::with_capacity(nsects as usize);
    for _ in 0..nsects {
        section_cmds.push(parse_section64(buf));
    }

    // バイト境界は8に揃えられているので
    // その分をskipする
    let parsed = buf.pos() - start_pos;
    let alignment = (8 - (parsed % 8)) % 8;
    buf.skip(alignment);

    (segment_cmd, section_cmds)
}

fn parse_section64(buf: &mut Reader) -> Section64 {
    let sectname = buf.read_fixed_size_string(16);
    let segname = buf.read_fixed_size_string(16);
    let addr = buf.read_u64();
    let size = buf.read_u64();
    let offset = buf.read_u32();
    let align = buf.read_u32();
    let reloff = buf.read_u32();
    let nreloc = buf.read_u32();

    let flags_n = buf.read_u32();
    let sect_type = SectionType::from_u32(flags_n & SectionType::BIT_MASK);
    let sect_attrs = SectionAttrs::from_u32(flags_n & SectionAttrs::BIT_MASK);

    let reserved1 = buf.read_u32();
    let reserved2 = buf.read_u32();
    let reserved3 = buf.read_u32();

    Section64 {
        sectname,
        segname,
        addr,
        size,
        offset,
        align,
        reloff,
        nreloc,
        flags: (sect_attrs, sect_type),
        reserved1,
        reserved2,
        reserved3,
    }
}
