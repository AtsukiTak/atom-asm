use atom_macho::load_command::segment64::{
    Section64, SectionAttr, SectionAttrs, SectionType, SegmentCommand64,
};

pub struct SegmentCommand64Builder {
    cmd: SegmentCommand64,
}

impl SegmentCommand64Builder {
    pub fn new() -> Self {
        SegmentCommand64Builder {
            cmd: SegmentCommand64 {
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
            },
        }
    }

    pub fn add_section(&mut self, size: u64) -> &mut Self {
        self.cmd.cmdsize += Section64::SIZE;
        self.cmd.nsects += 1;
        self.cmd.vmsize += size;
        self.cmd.filesize += size;
        self
    }

    pub fn fileoff(&mut self, fileoff: u64) -> &mut Self {
        self.cmd.fileoff = fileoff;
        self
    }

    pub fn flags(&mut self, flags: u32) -> &mut Self {
        self.cmd.flags = flags;
        self
    }

    pub fn build(&self) -> SegmentCommand64 {
        self.cmd.clone()
    }
}

pub struct Section64Builder {
    cmd: Section64,
}

impl Section64Builder {
    pub fn new() -> Self {
        Section64Builder {
            cmd: Section64 {
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
            },
        }
    }

    /// configure this section as __TEXT.__text section
    pub fn text_section(&mut self) -> &mut Self {
        self.cmd.sectname = "__text".to_string();
        self.cmd.segname = "__TEXT".to_string();

        // __TEXT.__text セクションではだいたい
        // ↓の2つのattrを有効にする
        let (ref mut attrs, _) = self.cmd.flags;
        attrs.push(SectionAttr::SomeInstructions);
        attrs.push(SectionAttr::PureInstructions);

        // __textセクションではalignは1 (2^0)
        self.cmd.align = 0;

        self
    }

    /// set memory address of this section
    pub fn addr(&mut self, addr: u64) -> &mut Self {
        self.cmd.addr = addr;
        self
    }

    /// set size in bytes of this section
    pub fn size(&mut self, size: u64) -> &mut Self {
        self.cmd.size = size;
        self
    }

    /// set file offset of this section
    pub fn offset(&mut self, offset: u32) -> &mut Self {
        self.cmd.offset = offset;
        self
    }

    /// set the file offset of the first relocation entry for this section.
    pub fn reloff(&mut self, reloff: u32) -> &mut Self {
        self.cmd.reloff = reloff;
        self
    }

    /// set the number of relocation entry for this section
    pub fn nreloc(&mut self, nreloc: u32) -> &mut Self {
        self.cmd.nreloc = nreloc;
        self
    }

    pub fn build(&self) -> Section64 {
        self.cmd.clone()
    }
}
