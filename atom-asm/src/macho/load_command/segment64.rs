use atom_macho::load_command::segment64::{
    Section64, SectionAttr, SectionAttrs, SectionType, SegmentCommand64,
};

pub trait SegmentCommand64Configure {
    fn new() -> Self;

    fn add_section(&mut self, size: u64);
}

impl SegmentCommand64Configure for SegmentCommand64 {
    fn new() -> Self {
        SegmentCommand64 {
            cmd: SegmentCommand64::TYPE,
            cmdsize: SegmentCommand64::SIZE,
            // always "" in object file
            segname: "".to_string(),
            // always 0 in object file
            vmaddr: 0,
            vmsize: 0,
            fileoff: 0,
            filesize: 0,
            // always 7 (rwx) in object file
            maxprot: 7,
            // always 7 (rwx) in object file
            initprot: 7,
            nsects: 0,
            flags: 0,
        }
    }

    fn add_section(&mut self, size: u64) {
        self.cmdsize += Section64::SIZE;
        self.nsects += 1;
        self.vmsize += size;
        self.filesize += size;
    }
}

pub trait Section64Configure {
    fn new() -> Self;
    fn text_section(&mut self);
}

impl Section64Configure for Section64 {
    fn new() -> Self {
        Section64 {
            sectname: "".to_string(),
            segname: "".to_string(),
            addr: 0,
            size: 0,
            offset: 0,
            align: 0,
            reloff: 0,
            nreloc: 0,
            flags: (SectionAttrs::new(), SectionType::Regular),
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
        }
    }

    /// configure this section as __TEXT.__text section
    fn text_section(&mut self) {
        self.sectname = "__text".to_string();
        self.segname = "__TEXT".to_string();

        // __TEXT.__text セクションではだいたい
        // ↓の2つのattrを有効にする
        let (ref mut attrs, _) = self.flags;
        attrs.push(SectionAttr::SomeInstructions);
        attrs.push(SectionAttr::PureInstructions);

        // __textセクションではalignは1 (2^0)
        self.align = 0;
    }
}
