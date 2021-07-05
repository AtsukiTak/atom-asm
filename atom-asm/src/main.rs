mod macho;
mod num;

use self::macho::{
    object::Object,
    section::{DataSection, Reloc, TextSection},
    symbol::Symbol,
};
use std::fs::{File, OpenOptions};

fn main() {
    let mut file = open_file("mach.o");

    let mut obj = Object::new();

    obj.sections.text = Some(TextSection {
        bytes: vec![
            0xB8, 0x04, 0x00, 0x00, 0x02, 0xBF, 0x01, 0x00, 0x00, 0x00, 0x48, 0x8D, 0x35, 0x00,
            0x00, 0x00, 0x00, 0xBA, 0x0E, 0x00, 0x00, 0x00, 0x0F, 0x05, 0xB8, 0x01, 0x00, 0x00,
            0x02, 0xBF, 0x00, 0x00, 0x00, 0x00, 0x0F, 0x05,
        ],
        symbols: vec![Symbol::Ref {
            name: "start".to_string(),
            addr: 0,
            ext: true,
        }],
        relocs: vec![Reloc {
            addr: 13,
            symbol: "msg".to_string(),
            pcrel: true,
            len: 2,
        }],
    });

    obj.sections.data = Some(DataSection {
        bytes: vec![
            0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x0A,
        ],
        symbols: vec![
            Symbol::Ref {
                name: "msg".to_string(),
                addr: 0,
                ext: true,
            },
            Symbol::Abs {
                name: "len".to_string(),
                val: 14,
                ext: true,
            },
        ],
        relocs: vec![],
    });

    obj.write_into(&mut file);
}

fn open_file(path: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap()
}
