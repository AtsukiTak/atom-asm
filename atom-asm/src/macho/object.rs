use super::{section::Sections, symbol::Symbol};
use atom_macho::{
    header::{CpuSubTypeX86_64, CpuType, FileType, Flags, Header64, Magic},
    load_command::{
        segment64::{Section64, SegmentCommand64},
        symtab::SymtabCommand,
    },
    nlist::{NList64, NType, NTypeField},
    reloc::{RelocLength, RelocationInfo},
    string_table::StringTable,
};
use std::io::Write;

pub struct Object {
    pub sections: Sections,
}

impl Object {
    pub fn new() -> Self {
        Object {
            sections: Sections::new(),
        }
    }

    pub fn write_into<W: Write>(&self, write: &mut W) {
        // write Header64
        self.gen_header64().write_into(write);

        // write SegmentCommand64
        self.gen_segment_command64().write_into(write);

        // write Section64
        self.gen_section64()
            .iter()
            .for_each(|sect| sect.write_into(write));

        // write SymtabCommand
        self.gen_symtab_command().write_into(write);

        // write SectionData
        self.sections.write_into(write);

        // create StringTable (write later)
        let stab = self.gen_string_table();

        // create Vec<NList64> (write later)
        let symbols = self.gen_nlist64s(&stab);

        // write RelocationInfo
        self.gen_relocation_infos(&symbols, &stab)
            .iter()
            .for_each(|reloc| reloc.write_into(write));

        // write Vec<NList64>
        symbols.iter().for_each(|sym| sym.write_into(write));

        // write StringTable
        write.write_all(stab.as_ref()).unwrap();
    }

    fn gen_header64(&self) -> Header64 {
        Header64 {
            magic: Magic::Magic64,
            cpu_type: CpuType::X86_64(CpuSubTypeX86_64::All),
            file_type: FileType::Object,
            n_cmds: 2,
            size_of_cmds: SegmentCommand64::SIZE
                + self.sections.len() * Section64::SIZE
                + SymtabCommand::SIZE,
            flags: Flags::new(),
            reserved: 0,
        }
    }

    fn gen_segment_command64(&self) -> SegmentCommand64 {
        SegmentCommand64 {
            cmd: SegmentCommand64::TYPE,
            cmdsize: SegmentCommand64::SIZE + self.sections.len() * Section64::SIZE,
            segname: "".to_string(),
            vmaddr: 0,
            vmsize: self.sections.vm_size(),
            fileoff: (Header64::SIZE
                + SegmentCommand64::SIZE
                + self.sections.len() * Section64::SIZE
                + SymtabCommand::SIZE) as u64,
            filesize: self.sections.file_size() as u64,
            maxprot: 7,
            initprot: 7,
            nsects: self.sections.len(),
            flags: 0,
        }
    }

    fn gen_section64(&self) -> Vec<Section64> {
        let mut vmaddr = 0_u64;
        let mut data_start = Header64::SIZE
            + SegmentCommand64::SIZE
            + self.sections.len() * Section64::SIZE
            + SymtabCommand::SIZE;
        let mut reloc_start = data_start + self.sections.file_size();

        self.sections
            .iter()
            .map(|section| {
                let addr = vmaddr;
                vmaddr += section.vm_size();

                let offset = data_start;
                data_start += section.file_size();

                let reloff = reloc_start;
                reloc_start += RelocationInfo::SIZE * section.num_relocs();

                section.gen_section64(addr, offset, reloff)
            })
            .collect()
    }

    fn gen_symtab_command(&self) -> SymtabCommand {
        let symoff = Header64::SIZE
            + SegmentCommand64::SIZE
            + self.sections.len() * Section64::SIZE
            + SymtabCommand::SIZE
            + self.sections.file_size()
            + self.sections.num_relocs();
        let nsyms = self.sections.num_symbols();
        let stroff = symoff + nsyms * NList64::SIZE;
        let strsize = self.sections.sum_symbol_names_len();

        SymtabCommand {
            cmd: SymtabCommand::TYPE,
            cmdsize: SymtabCommand::SIZE,
            symoff,
            nsyms,
            stroff,
            strsize,
        }
    }

    fn gen_relocation_infos(&self, symbols: &[NList64], stab: &StringTable) -> Vec<RelocationInfo> {
        fn get_sym_idx(symbols: &[NList64], stab: &StringTable, name: &str) -> u32 {
            symbols
                .iter()
                .enumerate()
                .find(|(_, sym)| stab.get(sym.n_strx as usize) == name)
                .map(|(idx, _)| idx as u32)
                .unwrap()
        }

        self.sections
            .relocs()
            .map(|reloc| RelocationInfo {
                r_address: reloc.addr,
                r_symbolnum: get_sym_idx(symbols, stab, reloc.symbol.as_str()),
                r_pcrel: reloc.pcrel,
                r_length: RelocLength::from_u32(reloc.len as u32),
                r_extern: true,
                r_type: 0,
            })
            .collect()
    }

    fn gen_nlist64s(&self, stab: &StringTable) -> Vec<NList64> {
        fn get_strx(stab: &StringTable, name: &str) -> u32 {
            stab.as_ref()
                .split(|byte| *byte == 0)
                .enumerate()
                .find(|(_, s)| *s == name.as_bytes())
                .map(|(idx, _)| idx as u32)
                .unwrap()
        }

        self.sections
            .iter()
            .enumerate()
            .flat_map(|(i, section)| section.symbols().iter().map(move |sym| (i, sym)))
            .map(|(sect_idx, symbol)| match symbol {
                Symbol::Undef { name } => NList64 {
                    n_strx: get_strx(stab, name.as_str()),
                    n_type: NTypeField::Norm {
                        n_pext: false,
                        n_type: NType::Undf,
                        n_ext: true,
                    },
                    n_sect: 0,
                    n_desc: 0,
                    n_value: 0,
                },
                Symbol::Abs { name, val, ext } => NList64 {
                    n_strx: get_strx(stab, name.as_str()),
                    n_type: NTypeField::Norm {
                        n_pext: false,
                        n_type: NType::Abs,
                        n_ext: *ext,
                    },
                    n_sect: sect_idx as u8,
                    n_desc: 0,
                    n_value: *val,
                },
                Symbol::Ref { name, addr, ext } => NList64 {
                    n_strx: get_strx(stab, name.as_str()),
                    n_type: NTypeField::Norm {
                        n_pext: false,
                        n_type: NType::Sect,
                        n_ext: *ext,
                    },
                    n_sect: sect_idx as u8,
                    n_desc: 0,
                    n_value: *addr,
                },
            })
            .collect()
    }

    fn gen_string_table(&self) -> StringTable {
        let mut stab = StringTable::with_null();
        self.sections
            .symbols()
            .for_each(|sym| stab.push_with_null(sym.name()));
        stab
    }
}
