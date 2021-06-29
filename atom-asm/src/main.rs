mod macho;
mod num;

use self::macho::object::Object;
use std::fs::{File, OpenOptions};

fn main() {
    let mut file = open_file("");

    let obj = Object::new();
    obj.write_into(&mut file);

    todo!()
}

fn open_file(path: &str) -> File {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .unwrap()
}
