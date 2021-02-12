use macho_parser::{Buffer, MachO};
use std::fs::File;

fn main() {
    let mut file = get_file();
    let mut buf = Buffer::new(&mut file);

    let macho = MachO::parse(&mut buf);
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
