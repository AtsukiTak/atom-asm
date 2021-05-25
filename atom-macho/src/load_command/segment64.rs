use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SegmentCommand64 {
    /// SegmentCommand64::TYPE
    pub cmd: u32,
    /// includes sizeof Section64 structs
    pub cmdsize: u32,
    /// segment name. 16byte
    pub segname: String,
    /// memory address of this segment
    pub vmaddr: u64,
    /// memory size of this segment
    pub vmsize: u64,
    /// file offset of this segment
    pub fileoff: u64,
    /// amount to map from the file
    pub filesize: u64,
    /// maximum VM protection
    pub maxprot: i32,
    /// initial VM protection
    pub initprot: i32,
    /// number of sections in segment
    pub nsects: u32,
    /// flags
    pub flags: u32,
}

impl SegmentCommand64 {
    pub const TYPE: u32 = 0x19;

    /// Byte size of `SegmentCommand64` command.
    /// This does not include `Section64` command size.
    /// So this is constant.
    #[rustfmt::skip]
    pub const SIZE: u32 =
        4       // cmd
        + 4     // cmdsize
        + 16    // segname
        + 8     // vmaddr
        + 8     // vmsize
        + 8     // fileoff
        + 8     // filesize
        + 4     // maxprot
        + 4     // initprot
        + 4     // nsects
        + 4; // flags
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section64 {
    /// 16-byte string
    pub sectname: String,
    /// 16-byte string
    pub segname: String,
    /// memory address of this section
    pub addr: u64,
    /// size in bytes of this section
    pub size: u64,
    /// file offset of this section
    pub offset: u32,
    /// section alignment (power of 2)
    pub align: u32,
    /// file offset of the first relocation entry for this section
    pub reloff: u32,
    /// number of relocation entries for this section
    pub nreloc: u32,
    /// represented as u32.
    /// higher 3 bytes represent SectionAttrs,
    /// lower 1 byte represent SectionType.
    pub flags: (SectionAttrs, SectionType),
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: u32,
}

impl Section64 {
    #[rustfmt::skip]
    pub const SIZE: u32 =
        16          // sectname
            + 16    // segname
            + 8     // addr
            + 8     // size
            + 4     // offset
            + 4     // align
            + 4     // reloff
            + 4     // nreloc
            + 4     // flags
            + 4     // reserved1
            + 4     // reserved2
            + 4; // reserved3
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Regular = 0x0,
    Zerofill = 0x1,
    CstringLiterals = 0x2,
    FourByteLiterals = 0x3,
    EightByteLiterals = 0x4,
    LiteralPointers = 0x5,
    Coalesced = 0xB,
}

impl SectionType {
    pub const BIT_MASK: u32 = 0x000000ff;

    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap_or_else(|| panic!("Unsupported section type 0x{:X}", n))
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionAttr {
    /// This section contains only executable machine instructions. The standard tools set this
    /// flag for the sections __TEXT,__text, __TEXT,__symbol_stub, and __TEXT,__picsymbol_stub.
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
    /// section contains some executable machine instructions.
    SomeInstructions = 0x00000400,
    /// section has external relocation entries.
    ExtReloc = 0x00000200,
    /// section has local relocation entries.
    LocReloc = 0x00000100,
}

impl SectionAttr {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n)
            .unwrap_or_else(|| panic!("Unsupported section attribute 0x{:X}", n))
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SectionAttrs {
    attrs: Vec<SectionAttr>,
}

impl SectionAttrs {
    pub const BIT_MASK: u32 = 0xffffff00;

    pub fn new() -> SectionAttrs {
        SectionAttrs { attrs: Vec::new() }
    }

    pub fn push(&mut self, attr: SectionAttr) {
        self.attrs.push(attr);
    }

    pub fn from_u32(flags: u32) -> Self {
        let mut attrs = SectionAttrs::new();
        for i in 8..=31 {
            let attr_n = flags & (1 << i);
            if attr_n != 0 {
                attrs.push(SectionAttr::from_u32(attr_n));
            }
        }
        attrs
    }

    pub fn to_u32(&self) -> u32 {
        let mut n = 0;
        for attr in self.attrs.iter() {
            n |= attr.to_u32();
        }
        n
    }
}

impl fmt::Debug for SectionAttrs {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.attrs.iter()).finish()
    }
}
