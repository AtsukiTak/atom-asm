use super::MachO;
use atom_macho::{
    header::Header64,
    load_command::{
        segment64::{Section64, SegmentCommand64},
        symtab::SymtabCommand,
    },
    nlist::NList64,
    string_table::StringTable,
};
use byteorder::{NativeEndian, WriteBytesExt};

pub trait Write: std::io::Write + WriteBytesExt {
    fn write_macho(&mut self, macho: &MachO) {
        self.write_header(&macho.header);

        self.write_segment_cmd(&macho.segment_cmd.0, &macho.segment_cmd.1);
        self.write_symtab_cmd(&macho.symtab_cmd);

        for section in macho.sect_datas.iter() {
            self.write_section(&section);
        }

        for nlist in macho.nlists.iter() {
            self.write_nlist(nlist);
        }

        self.write_string_table(&macho.string_table);
    }

    fn write_header(&mut self, header: &Header64) {
        self.write_u32_native(header.magic.to_u32());
        let (cpu_type, cpu_subtype) = header.cpu_type.to_i32_i32();
        self.write_i32_native(cpu_type);
        self.write_i32_native(cpu_subtype);
        self.write_u32_native(header.file_type.to_u32());
        self.write_u32_native(header.n_cmds);
        self.write_u32_native(header.size_of_cmds);
        self.write_u32_native(header.flags.to_u32());
        self.write_u32_native(header.reserved);
    }

    fn write_segment_cmd(&mut self, segment_cmd: &SegmentCommand64, section_cmds: &[Section64]) {
        // write SegmentCommand64
        self.write_u32_native(segment_cmd.cmd);
        self.write_u32_native(segment_cmd.cmdsize);
        self.write_fixed_size_string(segment_cmd.segname.as_str(), 16);
        self.write_u64_native(segment_cmd.vmaddr);
        self.write_u64_native(segment_cmd.vmsize);
        self.write_u64_native(segment_cmd.fileoff);
        self.write_u64_native(segment_cmd.filesize);
        self.write_i32_native(segment_cmd.maxprot);
        self.write_i32_native(segment_cmd.initprot);
        self.write_u32_native(segment_cmd.nsects);
        self.write_u32_native(segment_cmd.flags);

        // write Section64
        for section_cmd in section_cmds {
            self.write_fixed_size_string(section_cmd.sectname.as_str(), 16);
            self.write_fixed_size_string(section_cmd.segname.as_str(), 16);
            self.write_u64_native(section_cmd.addr);
            self.write_u64_native(section_cmd.size);
            self.write_u32_native(section_cmd.offset);
            self.write_u32_native(section_cmd.align);
            self.write_u32_native(section_cmd.reloff);
            self.write_u32_native(section_cmd.nreloc);
            self.write_u32_native(section_cmd.flags.0.to_u32() | section_cmd.flags.1.to_u32());
            self.write_u32_native(section_cmd.reserved1);
            self.write_u32_native(section_cmd.reserved2);
            self.write_u32_native(section_cmd.reserved3);
        }
    }

    fn write_symtab_cmd(&mut self, symtab_cmd: &SymtabCommand) {
        self.write_u32_native(symtab_cmd.cmd);
        self.write_u32_native(symtab_cmd.cmdsize);
        self.write_u32_native(symtab_cmd.symoff);
        self.write_u32_native(symtab_cmd.nsyms);
        self.write_u32_native(symtab_cmd.stroff);
        self.write_u32_native(symtab_cmd.strsize);
    }

    fn write_section(&mut self, section: &[u8]) {
        self.write_all(section).unwrap();

        // 8 byte alignment
        if section.len() % 8 > 0 {
            let num_pad = 8 - section.len() % 8;
            let pad_bytes = &[0u8; 8][0..num_pad];
            self.write_all(pad_bytes).unwrap();
        }
    }

    fn write_nlist(&mut self, nlist: &NList64) {
        self.write_u32_native(nlist.n_strx);
        self.write_u8(nlist.n_type.to_u8()).unwrap();
        self.write_u8(nlist.n_sect).unwrap();
        self.write_u16_native(nlist.n_desc);
        self.write_u64_native(nlist.n_value);
    }

    fn write_string_table(&mut self, string_table: &StringTable) {
        self.write_all(string_table.as_ref()).unwrap()
    }

    fn write_u16_native(&mut self, n: u16) {
        self.write_u16::<NativeEndian>(n).unwrap()
    }

    fn write_u32_native(&mut self, n: u32) {
        self.write_u32::<NativeEndian>(n).unwrap()
    }

    fn write_u64_native(&mut self, n: u64) {
        self.write_u64::<NativeEndian>(n).unwrap()
    }

    fn write_i32_native(&mut self, n: i32) {
        self.write_i32::<NativeEndian>(n).unwrap()
    }

    fn write_fixed_size_string(&mut self, s: &str, size: usize) {
        assert!(s.is_ascii());
        assert!(s.len() <= size);

        let mut buf = vec![0u8; size];

        for (i, c) in s.chars().enumerate() {
            buf[i] = c as u8;
        }

        self.write_all(&buf).unwrap()
    }
}

impl<T: std::io::Write> Write for T {}
