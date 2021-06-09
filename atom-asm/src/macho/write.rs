use crate::{macho::MachO, num::NumExt as _};
use atom_macho::string_table::StringTable;

pub trait Write: std::io::Write + Sized {
    fn write_macho(&mut self, macho: &MachO) {
        macho.header.write_into(self);

        macho.segment_cmd.0.write_into(self);
        for section in macho.segment_cmd.1 {
            section.write_into(self);
        }

        macho.symtab_cmd.write_into(self);

        for section in macho.sect_datas {
            self.write_section(&section);
        }

        for nlist in macho.nlists {
            nlist.write_into(self);
        }

        self.write_string_table(&macho.string_table);
    }

    fn write_section(&mut self, section: &[u8]) {
        self.write_all(section).unwrap();

        let aligned_len = section.len().aligned(8);
        let pad_bytes = &[0u8; 7][0..aligned_len - section.len()];
        self.write_all(pad_bytes).unwrap();
    }

    fn write_string_table(&mut self, string_table: &StringTable) {
        self.write_all(string_table.as_ref()).unwrap()
    }
}

impl<T: std::io::Write> Write for T {}
