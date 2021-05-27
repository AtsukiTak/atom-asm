use crate::io::{Endian, ReadExt, WriteExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelocationInfo {
    /// offset in the section to what is being relocated
    r_address: i32,
    r_symbolnum: u32,
    r_pcrel: bool,
    r_length: RelocLength,
    r_extern: bool,
    /// if not 0, machine specific relocation type
    r_type: u8,
}

impl RelocationInfo {
    /// size in bytes
    pub const SIZE: u32 = 8;

    pub fn read_from_in<R: Read>(read: &mut R, endian: Endian) -> RelocationInfo {
        let r_address = read.read_i32_in(endian);

        let infos = read.read_u32_in(endian);
        let r_symbolnum = infos & 0xFFFFFF00;
        let r_pcrel = infos & 0x00000080 == 0x00000080;
        let r_length = RelocLength::from_u32(infos & 0x00000060 >> 5).unwrap();
        let r_extern = infos & 0x00000010 == 0x00000010;
        let r_type = (infos & 0x0000000F) as u8;

        RelocationInfo {
            r_address,
            r_symbolnum,
            r_pcrel,
            r_length,
            r_extern,
            r_type,
        }
    }

    pub fn write_into(self, write: &mut impl Write) {
        write.write_i32_native(self.r_address);

        let mut infos: u32 = 0;
        infos |= self.r_symbolnum;
        infos |= (self.r_pcrel as u32) * 0x00000080;
        infos |= (self.r_length as u32) << 5;
        infos |= self.r_type as u32;
        write.write_u32_native(infos);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum RelocLength {
    Byte = 0,
    Word = 1,
    Long = 2,
    Quad = 3,
}

impl RelocLength {
    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_relocation_info() {
        let reloc = RelocationInfo {
            r_address: 42,
            r_symbolnum: 0x00323100,
            r_pcrel: true,
            r_length: RelocLength::Byte,
            r_extern: false,
            r_type: 0,
        };

        let mut buf = Vec::new();
        reloc.write_into(&mut buf);

        let read_reloc = RelocationInfo::read_from_in(&mut buf.as_slice(), Endian::NATIVE);

        assert_eq!(read_reloc, reloc);
    }
}
