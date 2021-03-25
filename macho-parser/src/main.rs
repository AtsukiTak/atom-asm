use macho_types::{Buffer, MachO};
use std::{fs::File, io::Read as _};

fn main() {
    let mut file = get_file();
    let mut vec = Vec::new();
    file.read_to_end(&mut vec).unwrap();

    let mut buf = Buffer::new(vec);

    let macho = MachO::parse(&mut buf);
    dbg!(macho);

    // 残り
    buf.get_full_slice()
        .iter()
        .skip(buf.pos())
        .for_each(|byte| print!("{:X} ", byte));
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
