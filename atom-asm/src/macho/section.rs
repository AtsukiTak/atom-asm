use super::symbol::Symbol;
use crate::num::NumExt as _;
use atom_macho::load_command::segment64::{Section64, SectionAttr, SectionAttrs, SectionType};
use std::io::Write;

pub struct Sections {
    pub text_sect: Option<TextSection>,
    pub data_sect: Option<DataSection>,
    pub bss_sect: Option<BssSection>,
}

impl Sections {
    pub fn new() -> Self {
        Sections {
            text_sect: None,
            data_sect: None,
            bss_sect: None,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a dyn Section> {
        let text_iter = self.text_sect.iter().map(|s| s as &dyn Section);
        let data_iter = self.data_sect.iter().map(|s| s as &dyn Section);
        let bss_iter = self.bss_sect.iter().map(|s| s as &dyn Section);
        text_iter.chain(data_iter).chain(bss_iter)
    }

    pub fn len(&self) -> u32 {
        self.text_sect.is_some() as u32
            + self.data_sect.is_some() as u32
            + self.bss_sect.is_some() as u32
    }

    pub fn vm_size(&self) -> u64 {
        self.iter().map(|s| s.vm_size()).sum()
    }

    pub fn file_size(&self) -> u32 {
        self.iter().map(|s| s.file_size()).sum::<u32>().aligned(8)
    }

    pub fn symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.iter().flat_map(|s| s.symbols())
    }

    pub fn relocs(&self) -> impl Iterator<Item = &Reloc> {
        self.iter().flat_map(|s| s.relocs())
    }

    pub fn num_relocs(&self) -> u32 {
        self.iter().map(|s| s.num_relocs()).sum()
    }

    pub fn num_symbols(&self) -> u32 {
        self.iter().map(|s| s.num_symbols()).sum()
    }

    pub fn sum_symbol_names_len(&self) -> u32 {
        self.iter().map(|s| s.sum_symbol_name_len()).sum()
    }

    /// write all section data
    pub fn write_into<W: Write>(&self, write: &mut W) {
        self.iter()
            .for_each(|sect| write.write_all(sect.file_data()).unwrap());
        let padding = [0u8; 7];
        let n_padding = self.file_size().padding(8) as usize;
        write.write_all(&padding[..n_padding]).unwrap();
    }
}

pub trait Section {
    fn vm_size(&self) -> u64;
    fn file_data(&self) -> &[u8];
    fn symbols(&self) -> &[Symbol];
    fn relocs(&self) -> &[Reloc];
    fn gen_section64(&self, vm_addr: u64, offset: u32, reloc_offset: u32) -> Section64;

    fn file_size(&self) -> u32 {
        self.file_data().len() as u32
    }

    fn num_symbols(&self) -> u32 {
        self.symbols().len() as u32
    }

    fn sum_symbol_name_len(&self) -> u32 {
        self.symbols()
            .iter()
            // +1 は null pointer のサイズ
            .map(|sym| sym.name().len() as u32 + 1)
            .sum()
    }

    fn num_relocs(&self) -> u32 {
        self.relocs().len() as u32
    }
}

pub struct TextSection {
    bytes: Vec<u8>,
    symbols: Vec<Symbol>,
    relocs: Vec<Reloc>,
}

impl Section for TextSection {
    fn vm_size(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn file_data(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    fn symbols(&self) -> &[Symbol] {
        &self.symbols[..]
    }

    fn relocs(&self) -> &[Reloc] {
        &self.relocs[..]
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

pub struct DataSection {
    bytes: Vec<u8>,
    symbols: Vec<Symbol>,
    relocs: Vec<Reloc>,
}

impl Section for DataSection {
    fn vm_size(&self) -> u64 {
        self.bytes.len() as u64
    }

    fn file_data(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    fn symbols(&self) -> &[Symbol] {
        &self.symbols[..]
    }

    fn relocs(&self) -> &[Reloc] {
        &self.relocs[..]
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

pub struct BssSection {
    size: u64,
    symbols: Vec<Symbol>,
}

impl Section for BssSection {
    fn vm_size(&self) -> u64 {
        self.size
    }

    fn file_data(&self) -> &[u8] {
        const EMPTY: &'static [u8] = &[];
        EMPTY
    }

    fn symbols(&self) -> &[Symbol] {
        &self.symbols[..]
    }

    fn relocs(&self) -> &[Reloc] {
        const EMPTY: &'static [Reloc] = &[];
        EMPTY
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
    pub addr: i32,
    pub symbol: String,
    pub pcrel: bool,
    // 0 => 1 byte, 1 => 2 byte, 2 => 4 byte, 3 => 8 byte
    pub len: u8,
}
