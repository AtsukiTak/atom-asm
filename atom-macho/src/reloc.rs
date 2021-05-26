use byteorder::{ByteOrder, NativeEndian, ReadBytesExt as _, WriteBytesExt as _};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{Read, Result, Write};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum RelocLength {
    Byte = 0,
    Word = 1,
    Long = 2,
    Quad = 3,
}

impl RelocationInfo {
    /// size in bytes
    pub const SIZE: u32 = 8;

    pub fn read_from<T: ByteOrder>(read: &mut impl Read) -> Result<RelocationInfo> {
        let r_address = read.read_i32::<T>()?;

        let infos = read.read_u32::<T>()?;
        let r_symbolnum = infos & 0xFFFFFF00;
        let r_pcrel = infos & 0x00000080 == 0x00000080;
        let r_length = RelocLength::from_u32(infos & 0x00000060 >> 5).unwrap();
        let r_extern = infos & 0x00000010 == 0x00000010;
        let r_type = (infos & 0x0000000F) as u8;

        Ok(RelocationInfo {
            r_address,
            r_symbolnum,
            r_pcrel,
            r_length,
            r_extern,
            r_type,
        })
    }

    pub fn write_into(self, write: &mut impl Write) -> Result<()> {
        write.write_i32::<NativeEndian>(self.r_address)?;

        let mut infos: u32 = 0;
        infos &= self.r_symbolnum;
        infos &= (self.r_pcrel as u32) * 0x00000080;
        infos &= (self.r_length as u32) << 5;
        infos &= self.r_type as u32;
        write.write_u32::<NativeEndian>(infos)?;

        Ok(())
    }
}
