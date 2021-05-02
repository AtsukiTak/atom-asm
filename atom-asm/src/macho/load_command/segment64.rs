use atom_macho::load_command::segment64::{
    Section64, SectionAttr, SectionAttrs, SectionType, Segment64,
};

pub struct Segment64Builder {
    cmd: Segment64,
}

impl Segment64Builder {
    pub fn new() -> Self {
        Segment64Builder {
            cmd: Segment64 {
                cmd_size: Segment64::cmd_size(),
                // always "" in object file
                seg_name: "".to_string(),
                // always 0 in object file
                vm_addr: 0,
                vm_size: 0,
                file_off: 0,
                file_size: 0,
                // always 7 (rwx) in object file
                max_prot: 7,
                // always 7 (rwx) in object file
                init_prot: 7,
                nsects: 0,
                flags: 0,
            },
        }
    }

    pub fn add_section(&mut self, size: u64) -> &mut Self {
        self.cmd.cmd_size += Section64::cmd_size();
        self.cmd.nsects += 1;
        self.cmd.vm_size += size;
        self.cmd.file_size += size;
        self
    }

    pub fn file_off(&mut self, file_off: u64) -> &mut Self {
        self.cmd.file_off = file_off;
        self
    }

    pub fn flags(&mut self, flags: u32) -> &mut Self {
        self.cmd.flags = flags;
        self
    }

    pub fn build(&self) -> Segment64 {
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
                sect_name: "".to_string(),
                seg_name: "".to_string(),
                addr: 0,
                size: 0,
                offset: 0,
                align: 0,
                reloff: 0,
                nreloc: 0,
                flags: (SectionAttrs::new(), SectionType::Regular),
                reserved_1: 0,
                reserved_2: 0,
                reserved_3: 0,
            },
        }
    }

    /// configure this section as __TEXT.__text section
    pub fn text_section(&mut self) -> &mut Self {
        self.cmd.sect_name = "__text".to_string();
        self.cmd.seg_name = "__TEXT".to_string();

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
