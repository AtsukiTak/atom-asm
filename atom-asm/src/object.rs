pub struct Object {
    pub sections: Sections,
}

impl Object {
    pub fn new() -> Self {
        Object {
            sections: Sections {
                text: None,
                data: None,
                bss: None,
            },
        }
    }

    pub fn sections(&self) -> &Sections {
        &self.sections
    }
}

pub struct Sections {
    pub text: Option<TextSection>,
    pub data: Option<DataSection>,
    pub bss: Option<BssSection>,
}

impl Sections {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = SectionRef<'a>> {
        let text_iter = self.text.iter().map(SectionRef::Text);
        let data_iter = self.data.iter().map(SectionRef::Data);
        let bss_iter = self.bss.iter().map(SectionRef::Bss);
        text_iter.chain(data_iter).chain(bss_iter)
    }

    pub fn len(&self) -> u32 {
        self.text.is_some() as u32 + self.data.is_some() as u32 + self.bss.is_some() as u32
    }
}

pub enum SectionRef<'a> {
    Text(&'a TextSection),
    Data(&'a DataSection),
    Bss(&'a BssSection),
}

impl<'a> SectionRef<'a> {
    pub fn vm_size(&self) -> u64 {
        use SectionRef::*;

        match self {
            Text(text) => text.bytes.len() as u64,
            Data(data) => data.bytes.len() as u64,
            Bss(bss) => bss.size,
        }
    }

    pub fn file_data(&self) -> &[u8] {
        use SectionRef::*;

        const EMPTY: [u8; 0] = [];

        match self {
            Text(text) => text.bytes.as_slice(),
            Data(data) => data.bytes.as_slice(),
            Bss(_) => &EMPTY,
        }
    }

    pub fn file_size(&self) -> u32 {
        self.file_data().len() as u32
    }

    pub fn symbols(&self) -> &[Symbol] {
        use SectionRef::*;

        match self {
            Text(text) => text.symbols.as_slice(),
            Data(data) => data.symbols.as_slice(),
            Bss(bss) => bss.symbols.as_slice(),
        }
    }

    pub fn relocs(&self) -> &[Reloc] {
        use SectionRef::*;

        const EMPTY: [Reloc; 0] = [];

        match self {
            Text(text) => text.relocs.as_slice(),
            Data(data) => data.relocs.as_slice(),
            Bss(_) => &EMPTY,
        }
    }
}

pub trait Section {
    fn vm_size(&self) -> u64;
    fn file_data(&self) -> &[u8];
    fn symbols(&self) -> &[Symbol];
    fn relocs(&self) -> &[Reloc];

    fn file_size(&self) -> u32 {
        self.file_data().len() as u32
    }
}

pub struct TextSection {
    pub bytes: Vec<u8>,
    pub symbols: Vec<Symbol>,
    pub relocs: Vec<Reloc>,
}

pub struct DataSection {
    pub bytes: Vec<u8>,
    pub symbols: Vec<Symbol>,
    pub relocs: Vec<Reloc>,
}

pub struct BssSection {
    pub size: u64,
    pub symbols: Vec<Symbol>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Symbol {
    Undef { name: String },
    Abs { name: String, val: u64, ext: bool },
    Ref { name: String, addr: u64, ext: bool },
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Symbol::Undef { name } => name.as_str(),
            Symbol::Abs { name, .. } => name.as_str(),
            Symbol::Ref { name, .. } => name.as_str(),
        }
    }
}
