mod macho;
mod num;

use std::fs::{File, OpenOptions};

fn main() {
    todo!()
}

fn open_file(path: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap()
}
