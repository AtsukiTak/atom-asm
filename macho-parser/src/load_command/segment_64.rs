use crate::Buffer;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment64 {
    cmd_size: u32,
    seg_name: String,
    vm_addr: u64,
    vm_size: u64,
    file_off: u64,
    file_size: u64,
    max_prot: i32,
    init_prot: i32,
    flags: u32,
    sections: Vec<Section64>,
}

impl Segment64 {
    pub fn parse(buf: &mut Buffer) -> Self {
        let start_pos = buf.pos() - 4;

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
            sections.push(Section64::parse(buf));
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
        let alignment = 8 - (parsed % 8);
        buf.skip(alignment);

        command
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section64 {
    sect_name: String,
    seg_name: String,
    addr: u64,
    size: u64,
    offset: u32,
    align: u32,
    reloff: u32,
    nreloc: u32,
    sect_type: SectionType,
    sect_attrs: SectionAttrs,
}

impl Section64 {
    fn parse(buf: &mut Buffer) -> Self {
        let sect_name = buf.read_fixed_size_string(16);
        let seg_name = buf.read_fixed_size_string(16);
        let addr = buf.read_u64();
        let size = buf.read_u64();
        let offset = buf.read_u32();
        let align = buf.read_u32();
        let reloff = buf.read_u32();
        let nreloc = buf.read_u32();
        let flags = buf.read_u32();

        let sect_type = SectionType::from_u32(flags & 0x000000ff);
        let sect_attrs = SectionAttrs::from_u32(flags & 0xffffff00);

        // skip "reserved" fields
        buf.skip(8);

        Section64 {
            sect_name,
            seg_name,
            addr,
            size,
            offset,
            align,
            reloff,
            nreloc,
            sect_type,
            sect_attrs,
        }
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Regular = 0x0,
    ZeroFill = 0x1,
    CStringLiterals = 0x2,
    FourByteLiterals = 0x3,
    EightByteLiterals = 0x4,
    LiteralPointers = 0x5,
}

impl SectionType {
    fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).expect("Unsupported section attribute")
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionAttr {
    PureInstructions = 0x80000000,
    SomeInstructions = 0x00000400,
}

impl SectionAttr {
    fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).expect("Unsupported section attribute")
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SectionAttrs {
    attrs: Vec<SectionAttr>,
}

impl SectionAttrs {
    fn from_u32(flags: u32) -> Self {
        let mut attrs = Vec::new();
        for i in 8..=31 {
            let attr_n = flags & (1 << i);
            if attr_n != 0 {
                attrs.push(SectionAttr::from_u32(attr_n));
            }
        }
        SectionAttrs { attrs }
    }
}

impl fmt::Debug for SectionAttrs {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.attrs.iter()).finish()
    }
}
