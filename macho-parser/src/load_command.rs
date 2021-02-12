use crate::Buffer;
use mach_object as macho;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadCommand {
    Segment64(Segment64),
    SymTab(SymTab),
}

impl LoadCommand {
    pub fn parse(buf: &mut Buffer) -> Self {
        let cmd_type_n = buf.read_u32();
        if cmd_type_n == macho::LC_SEGMENT_64 {
            LoadCommand::Segment64(Segment64::parse(buf))
        } else if cmd_type_n == macho::LC_SYMTAB {
            LoadCommand::SymTab(SymTab::parse(buf))
        } else {
            panic!("Unsupported cmd_type {}", cmd_type_n);
        }
    }
}

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
    flags: u32,
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
            flags,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymTab {
    cmd_size: u32,
}

impl SymTab {
    pub fn parse(buf: &mut Buffer) -> Self {
        let cmd_size = buf.read_u32();

        buf.skip((cmd_size - 4 - 4) as usize);

        SymTab { cmd_size }
    }
}
