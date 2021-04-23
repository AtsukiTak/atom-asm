mod buffer;
mod macho;

use crate::buffer::Buffer;
use std::{fs::File, io::Read as _};

fn main() {
    let mut file = get_file();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec).unwrap();

    let buf = Buffer::new(vec);

    // try parse macho
    if let Some(macho) = macho::parse_macho(&mut buf.clone()) {
        dbg!(macho);
    }
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
