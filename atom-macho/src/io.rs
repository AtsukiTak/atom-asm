use byteorder::{BigEndian, LittleEndian, NativeEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

#[cfg(target_endian = "little")]
impl Endian {
    pub const NATIVE: Endian = Endian::Little;
    pub const REVERSE: Endian = Endian::Big;
}

#[cfg(target_endian = "big")]
impl Endian {
    pub const NATIVE: Endian = Endian::Big;
    pub const REVERSE: Endian = Endian::Little;
}

pub trait ReadExt: Read + ReadBytesExt {
    fn read_i32_in(&mut self, endian: Endian) -> i32 {
        match endian {
            Endian::Little => self.read_i32::<LittleEndian>().unwrap(),
            Endian::Big => self.read_i32::<BigEndian>().unwrap(),
        }
    }

    fn read_u32_in(&mut self, endian: Endian) -> u32 {
        match endian {
            Endian::Little => self.read_u32::<LittleEndian>().unwrap(),
            Endian::Big => self.read_u32::<BigEndian>().unwrap(),
        }
    }
}

impl<T> ReadExt for T where T: Read {}

pub trait WriteExt: Write + WriteBytesExt {
    fn write_i32_native(&mut self, n: i32) {
        self.write_i32::<NativeEndian>(n).unwrap()
    }

    fn write_u32_native(&mut self, n: u32) {
        self.write_u32::<NativeEndian>(n).unwrap()
    }
}

impl<T> WriteExt for T where T: Write {}
