use byteorder::{NativeEndian, ReadBytesExt as _};
use std::{
    io::{Cursor, Read},
    sync::Arc,
};

#[derive(Clone, Debug)]
pub struct Bytes(Arc<Vec<u8>>);

impl Bytes {
    pub fn new(vec: Vec<u8>) -> Self {
        Bytes(Arc::new(vec))
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref().as_ref()
    }
}

impl std::ops::Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_ref()
    }
}

#[cfg(target_endian = "little")]
type ReverseEndian = byteorder::BigEndian;

#[cfg(target_endian = "big")]
type ReverseEndian = byteorder::LittleEndian;

#[derive(Clone, Debug)]
pub struct Reader {
    endian: Endian,
    buf: Cursor<Bytes>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Endian {
    Native,
    Reverse,
}

macro_rules! endian_read {
    ($self:expr, $func:ident) => {
        match $self.endian {
            Endian::Native => $self.buf.$func::<NativeEndian>().unwrap(),
            Endian::Reverse => $self.buf.$func::<ReverseEndian>().unwrap(),
        }
    };
}

impl Reader {
    /// バイト列から、新しいReaderを生成する
    /// EndianはNativeEndian
    pub fn new(bytes: Bytes) -> Self {
        Reader {
            endian: Endian::Native,
            buf: Cursor::new(bytes),
        }
    }

    pub fn set_reverse_endian(&mut self) {
        self.endian = Endian::Reverse;
    }

    pub fn pos(&self) -> usize {
        self.buf.position() as usize
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.buf.set_position(pos as u64);
    }

    pub fn skip(&mut self, n: usize) -> &mut Reader {
        self.buf.set_position(self.buf.position() + n as u64);
        self
    }

    /*
     * ===============
     * READ functions
     * ===============
     */
    pub fn read_u8(&mut self) -> u8 {
        self.buf.read_u8().unwrap()
    }

    pub fn read_u16(&mut self) -> u16 {
        endian_read!(self, read_u16)
    }

    pub fn read_i32(&mut self) -> i32 {
        endian_read!(self, read_i32)
    }

    pub fn read_u32(&mut self) -> u32 {
        endian_read!(self, read_u32)
    }

    pub fn read_u64(&mut self) -> u64 {
        endian_read!(self, read_u64)
    }

    pub fn read_bytes(&mut self, size: usize) -> &[u8] {
        let start = self.pos() as usize;
        let end = start + size;
        self.skip(size);

        &self.buf.get_ref()[start..end]
    }

    pub fn read_fixed_size_string(&mut self, len: usize) -> String {
        let mut buf = vec![0u8; len];

        self.buf.read_exact(&mut buf).unwrap();
        let buf = buf.split(|&b| b == 0).next().unwrap().to_vec();

        String::from_utf8(buf).unwrap()
    }
}
