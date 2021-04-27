mod hex;
mod macho;
mod reader;

use crate::{
    hex::HexVec,
    reader::{Bytes, Reader},
};
use atom_macho::load_command::LoadCommand;
use std::{fs::File, io::Read as _};

fn main() {
    let mut file = get_file();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec).unwrap();

    let bytes = Bytes::new(vec);
    let buf = Reader::new(bytes);

    // try parse macho
    if let Some(macho) = macho::parse_macho(&mut buf.clone()) {
        dbg!(&macho);
        macho
            .load_commands
            .iter()
            .filter_map(|cmd| match cmd {
                LoadCommand::Segment64(seg) => Some(seg),
                _ => None,
            })
            .flat_map(|seg| seg.sections.iter())
            .enumerate()
            .for_each(|(i, sec)| {
                let data = macho::parse_section(&buf.clone(), sec);
                println!("Section {} : {:?}", i, HexVec::new(&data));
            });
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
