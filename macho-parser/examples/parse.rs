use std::io::Cursor;

use macho_parser::{Buffer, MachO};

fn main() {
    let bytes = include_bytes!("./a.out");
    let mut cur = Cursor::new(bytes);
    let mut buf = Buffer::new(&mut cur);

    let macho = MachO::parse(&mut buf);
    dbg!(macho);
}
