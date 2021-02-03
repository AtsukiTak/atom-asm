use std::io::Cursor;

use macho_parser::parse;

fn main() {
    let bytes = include_bytes!("./a.out");
    let mut cur = Cursor::new(bytes);

    let macho = parse(&mut cur);
    dbg!(macho);
}
