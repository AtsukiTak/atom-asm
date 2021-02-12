use crate::Magic;
use byteorder::{NativeEndian, ReadBytesExt as _};
use num_traits::FromPrimitive as _;
use std::io::Read;

#[cfg(target_endian = "little")]
type ReverseEndian = byteorder::BigEndian;

#[cfg(target_endian = "big")]
type ReverseEndian = byteorder::LittleEndian;

pub struct Buffer<'a> {
    magic: Magic,
    buf: &'a mut dyn Read,
    pos: usize,
}

macro_rules! endian_read {
    ($self:expr, $func:ident) => {
        match $self.magic {
            Magic::Magic64 => $self.buf.$func::<NativeEndian>().unwrap(),
            Magic::Cigam64 => $self.buf.$func::<ReverseEndian>().unwrap(),
            _ => unimplemented!(),
        }
    };
}

impl<'a> Buffer<'a> {
    pub fn new(buf: &'a mut impl Read) -> Self {
        let magic_n = buf
            .read_u32::<NativeEndian>()
            .expect("Unable to read magic number");
        let magic = Magic::from_u32(magic_n).expect("Invalid magic number");

        Buffer { magic, buf, pos: 4 }
    }

    pub fn magic(&self) -> &Magic {
        &self.magic
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn skip(&mut self, n: usize) {
        let mut buf = vec![0u8; n];
        self.buf.read_exact(&mut buf[..n]).unwrap();
        self.pos += n;
    }

    pub fn read_i32(&mut self) -> i32 {
        self.pos += 4;
        endian_read!(self, read_i32)
    }

    pub fn read_u32(&mut self) -> u32 {
        self.pos += 4;
        endian_read!(self, read_u32)
    }

    pub fn read_u64(&mut self) -> u64 {
        self.pos += 8;
        endian_read!(self, read_u64)
    }

    pub fn read_fixed_size_string(&mut self, len: usize) -> String {
        self.pos += len;

        let mut buf = vec![0u8; len];

        self.buf.read_exact(&mut buf).unwrap();
        let buf = buf.split(|&b| b == 0).next().unwrap().to_vec();

        String::from_utf8(buf).unwrap()
    }
}