mod macho;
mod num;

use self::macho::{MachO, Write};
use std::fs::{File, OpenOptions};

fn main() {
    let mut macho = MachO::new();
    let section_num = macho.add_text_section(vec![0x66, 0xB8, 0x2A, 0x00, 0xC3]);
    macho.add_symbol("_main", section_num, 0, true);

    // dbg!(&macho);

    let mut file = open_file("atom.o");
    file.write_macho(&macho);
}

fn open_file(path: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap()
}
