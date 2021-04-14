use crate::ReadBuf;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment64 {
    pub cmd_size: u32,
    pub seg_name: String,
    pub vm_addr: u64,
    pub vm_size: u64,
    pub file_off: u64,
    pub file_size: u64,
    pub max_prot: i32,
    pub init_prot: i32,
    pub flags: u32,
    pub sections: Vec<Section64>,
}

impl Segment64 {
    pub const COMMAND: u32 = 0x19;

    // Byte size of `Segment64` command.
    // This does not include `Section64` command size.
    // So this is constant.
    #[rustfmt::skip]
    pub const fn cmd_size() -> u32 {
        4           // cmd
            + 4     // cmd_size
            + 16    // seg_name
            + 8     // vm_addr
            + 8     // vm_size
            + 8     // file_off
            + 8     // file_size
            + 4     // max_prot
            + 4     // init_prot
            + 4     // nsects
            + 4     // flags
    }

    pub fn parse(buf: &mut ReadBuf) -> Self {
        let start_pos = buf.pos();

        let cmd_type = buf.read_u32();
        if cmd_type != Self::COMMAND {
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
        let alignment = (8 - (parsed % 8)) % 8;
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
    data: Data,
}

impl Section64 {
    #[rustfmt::skip]
    pub const fn cmd_size() -> u32 {
        16          // sect_name
            + 16    // seg_name
            + 8     // addr
            + 8     // size
            + 4     // offset
            + 4     // align
            + 4     // reloff
            + 4     // nreloc
            + 4     // flags
            + 4     // reserved1
            + 4     // reserved2
            + 4     // reserved3
    }

    fn parse(buf: &mut ReadBuf) -> Self {
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

        // skip "reserved[1,2,3]" fields
        buf.skip(12);

        let data_start = offset as usize;
        let data_end = data_start + size as usize;
        let data_slice = &buf.get_full_slice()[data_start..data_end];
        let data = Data(data_slice.to_vec());

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
            data,
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
    Coalesced = 0xB,
}

impl SectionType {
    fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap_or_else(|| panic!("Unsupported section type 0x{:X}", n))
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionAttr {
    PureInstructions = 0x80000000,
    /// section contains coalesced symbols that are not to be
    /// in a ranlib table of contents
    NoToc = 0x40000000,
    /// ok to strip static symbols in this section in files with the MH_DYLDLINK flag
    StripStaticSyms = 0x20000000,
    /// blocks are live if they reference live blocks
    LiveSupport = 0x08000000,
    /// If a segment contains any sections marked with S_ATTR_DEBUG then all
    /// sections in that segment must have this attribute.  No section other than
    /// a section marked with this attribute may reference the contents of this
    /// section.  A section with this attribute may contain no symbols and must have
    /// a section type S_REGULAR.  The static linker will not copy section contents
    /// from sections with this attribute into its output file.  These sections
    /// generally contain DWARF debugging info.
    Debug = 0x02000000,
    SomeInstructions = 0x00000400,
}

impl SectionAttr {
    fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n)
            .unwrap_or_else(|| panic!("Unsupported section attribute 0x{:X}", n))
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

#[derive(Clone, PartialEq, Eq)]
pub struct Data(Vec<u8>);

impl fmt::Debug for Data {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("{:02X?}", self.0))
    }
}
