use macho_parser::parse;
use std::{
    fs::File,
    io::{Cursor, Read},
};

fn main() {
    let bytes = read_file(get_file());
    let mut cur = Cursor::new(&bytes[..]);

    let macho = parse(&mut cur);
    dbg!(macho);
}

fn get_file() -> File {
    match std::env::args().skip(1).next() {
        Some(s) => File::open(s).expect("file path is invalid"),
        None => {
            println!("target file's path is required.");
            std::process::exit(1)
        }
    }
}

fn read_file(mut file: File) -> Vec<u8> {
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}
