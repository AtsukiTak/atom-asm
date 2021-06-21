use super::symbol::Symbol;
use crate::num::NumExt as _;
use atom_macho::{
    header::{CpuSubTypeX86_64, CpuType, FileType, Flags, Header64, Magic},
    load_command::{
        segment64::{Section64, SectionAttr, SectionAttrs, SectionType, SegmentCommand64},
        symtab::SymtabCommand,
    },
    reloc::RelocationInfo,
};
use std::io::Write;

pub struct Object {
    text_sect: Option<TextSection>,
    data_sect: Option<DataSection>,
    bss_sect: Option<BssSection>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            text_sect: None,
            data_sect: None,
            bss_sect: None,
        }
    }

    pub fn text_sect(&mut self, text_sect: TextSection) {
        self.text_sect = Some(text_sect);
    }

    pub fn data_sect(&mut self, data_sect: DataSection) {
        self.data_sect = Some(data_sect);
    }

    pub fn bss_sect(&mut self, bss_sect: BssSection) {
        self.bss_sect = Some(bss_sect);
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        self.gen_header64().write_into(write);

        let segment_cmd = self.gen_segment_command64();
        segment_cmd.write_into(write);

        // write Section64
        let mut vmaddr = 0_u64;
        let mut sect_data_offset = segment_cmd.fileoff as u32;
        let mut reloc_offset = (segment_cmd.fileoff + segment_cmd.filesize.aligned(8)) as u32;

        for section in self.sections() {
            let addr = vmaddr;
            vmaddr += section.vm_size();

            let offset = sect_data_offset;
            sect_data_offset += section.file_size();

            let reloff = reloc_offset;
            reloc_offset += RelocationInfo::SIZE * section.num_relocs();

            section
                .gen_section64(addr, offset, reloff)
                .write_into(write);
        }
    }

    fn gen_header64(&self) -> Header64 {
        Header64 {
            magic: Magic::Magic64,
            cpu_type: CpuType::X86_64(CpuSubTypeX86_64::All),
            file_type: FileType::Object,
            n_cmds: 2,
            size_of_cmds: SegmentCommand64::SIZE
                + self.num_sections() * Section64::SIZE
                + SymtabCommand::SIZE,
            flags: Flags::new(),
            reserved: 0,
        }
    }

    fn gen_segment_command64(&self) -> SegmentCommand64 {
        SegmentCommand64 {
            cmd: SegmentCommand64::TYPE,
            cmdsize: SegmentCommand64::SIZE + self.num_sections() * Section64::SIZE,
            segname: "".to_string(),
            vmaddr: 0,
            vmsize: self.sections().map(|s| s.vm_size()).sum(),
            fileoff: (Header64::SIZE
                + SegmentCommand64::SIZE
                + self.num_sections() * Section64::SIZE
                + SymtabCommand::SIZE) as u64,
            filesize: self.sections().map(|s| s.file_size()).sum::<u32>() as u64,
            maxprot: 7,
            initprot: 7,
            nsects: self.num_sections(),
            flags: 0,
        }
    }

    fn num_sections(&self) -> u32 {
        self.text_sect.is_some() as u32
            + self.data_sect.is_some() as u32
            + self.bss_sect.is_some() as u32
    }

    fn sections<'a>(&'a self) -> impl Iterator<Item = &'a dyn Section> {
        self.text_sect
            .iter()
            .map(|s| s as &dyn Section)
            .chain(self.data_sect.iter().map(|s| s as &dyn Section))
            .chain(self.bss_sect.iter().map(|s| s as &dyn Section))
    }

    fn gen_symtab_command(&self, symoff: u32, stroff: u32) -> SymtabCommand {
        SymtabCommand {
            cmd: SymtabCommand::TYPE,
            cmdsize: SymtabCommand::SIZE,
            symoff,
            nsyms: self
                .sections()
                .map(|sect| sect.symbols().len() as u32)
                .sum(),
            stroff,
            strsize: self
                .sections()
                .map(|sect| {
                    sect.symbols()
                        .iter()
                        .map(|sym| sym.name().len() as u32 + 1)
                        .sum::<u32>()
                })
                .sum(),
        }
    }
}

trait Section {
    fn vm_size(&self) -> u64;
    fn file_size(&self) -> u32;
    fn symbols(&self) -> &[Symbol];
    fn num_relocs(&self) -> u32;
    fn gen_section64(&self, vm_addr: u64, offset: u32, reloc_offset: u32) -> Section64;
}

struct TextSection {
    bytes: Vec<u8>,
    symbols: Vec<Symbol>,
    relocs: Vec<Reloc>,
}

impl Section for TextSection {
    fn vm_size(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn file_size(&self) -> u32 {
        self.bytes.len() as u32
    }

    fn symbols(&self) -> &[Symbol] {
        &self.symbols[..]
    }

    fn num_relocs(&self) -> u32 {
        self.relocs.len() as u32
    }

    fn gen_section64(&self, addr: u64, offset: u32, reloff: u32) -> Section64 {
        let mut attrs = SectionAttrs::new();
        attrs.push(SectionAttr::SomeInstructions);
        attrs.push(SectionAttr::PureInstructions);
        if !self.relocs.is_empty() {
            attrs.push(SectionAttr::LocReloc);
            attrs.push(SectionAttr::ExtReloc);
        }

        Section64 {
            sectname: "__text".to_string(),
            segname: "__TEXT".to_string(),
            addr,
            size: self.bytes.len() as u64,
            offset,
            align: 0,
            reloff,
            nreloc: self.relocs.len() as u32,
            flags: (attrs, SectionType::Regular),
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
        }
    }
}

struct DataSection {
    bytes: Vec<u8>,
    symbols: Vec<Symbol>,
    relocs: Vec<Reloc>,
}

impl Section for DataSection {
    fn vm_size(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn file_size(&self) -> u32 {
        self.bytes.len() as u32
    }

    fn symbols(&self) -> &[Symbol] {
        &self.symbols[..]
    }

    fn num_relocs(&self) -> u32 {
        self.relocs.len() as u32
    }

    fn gen_section64(&self, addr: u64, offset: u32, reloff: u32) -> Section64 {
        let mut attrs = SectionAttrs::new();
        if !self.relocs.is_empty() {
            attrs.push(SectionAttr::LocReloc);
            attrs.push(SectionAttr::ExtReloc);
        }

        Section64 {
            sectname: "__data".to_string(),
            segname: "__DATA".to_string(),
            addr,
            size: self.bytes.len() as u64,
            offset,
            align: 0,
            reloff,
            nreloc: self.relocs.len() as u32,
            flags: (attrs, SectionType::Regular),
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
        }
    }
}

struct BssSection {
    size: u64,
    symbols: Vec<Symbol>,
}

impl Section for BssSection {
    fn vm_size(&self) -> u64 {
        self.size
    }

    fn file_size(&self) -> u32 {
        0
    }

    fn num_relocs(&self) -> u32 {
        0
    }

    fn symbols(&self) -> &[Symbol] {
        &self.symbols[..]
    }

    fn gen_section64(&self, addr: u64, _offset: u32, _reloff: u32) -> Section64 {
        Section64 {
            sectname: "__bss".to_string(),
            segname: "__DATA".to_string(),
            addr,
            size: self.size,
            offset: 0,
            align: 0,
            reloff: 0,
            nreloc: 0,
            flags: (SectionAttrs::new(), SectionType::Zerofill),
            reserved1: 0,
            reserved2: 0,
            reserved3: 0,
        }
    }
}

pub struct Reloc {
    /// offset from the start of the section to the
    /// item containing the address requiring relocation
    addr: i32,
    symbol: String,
    pcrel: bool,
    // 0 => 1 byte, 1 => 2 byte, 2 => 4 byte, 3 => 8 byte
    len: u8,
}
