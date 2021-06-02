use crate::io::{Endian, ReadExt, WriteExt};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelocationInfo {
    /// In MH_OBJECT files, this is an offset from the start of the section to the item containing
    /// the address requiring relocation.
    r_address: i32,
    /// Indicates symbol index if r_extern is true or section ordinal if r_extern is false.
    /// This field is set to R_ABS for relocation entries for absolute symbols, which need no
    /// relocation.
    r_symbolnum: u32,
    /// Indicates whether the item containing the address to be relocated is part of a CPU
    /// instruction that uses PC-relative addressing.
    ///
    /// For addresses contained in PC-relative instructions, the CPU adds the address of the
    /// instruction to the address contained in the instruction.
    r_pcrel: bool,
    r_length: RelocLength,
    /// Indicates whether the r_symbolnum field is an index into the symbol table (true) or a section
    /// number (false).
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

        // order of bit-field is basically determined by endian.
        // Mach-O specification does not clearly specify
        // memory layout of these fields. So we assume that
        // an order of bit-fields is determined by ordinal way (by endian).
        let (r_symbolnum, r_pcrel, r_length, r_extern, r_type) = if endian == Endian::Little {
            (
                infos & 0x00FF_FFFF,                                // r_symbolnum
                (infos & 0x0100_0000) > 0,                          // r_pcrel
                RelocLength::from_u32((infos & 0x0600_0000) >> 25), // r_length
                infos & 0x0800_0000 > 0,                            // r_extern
                ((infos & 0xF000_0000) >> 28) as u8,                // r_type
            )
        } else {
            (
                (infos & 0xFFFF_FF00) >> 8,                        // r_symbolnum
                infos & 0x0000_0080 > 0,                           // r_pcrel
                RelocLength::from_u32((infos & 0x0000_0060) >> 5), // r_length
                infos & 0x0000_0010 > 0,                           // r_extern
                (infos & 0x0000_000F) as u8,                       // r_type
            )
        };

        RelocationInfo {
            r_address,
            r_symbolnum,
            r_pcrel,
            r_length,
            r_extern,
            r_type,
        }
    }

    #[cfg(target_endian = "little")]
    pub fn write_into(self, write: &mut impl Write) {
        write.write_i32_native(self.r_address);

        let mut infos: u32 = 0;
        infos |= self.r_symbolnum;
        infos |= (self.r_pcrel as u32) * 0x0100_0000;
        infos |= (self.r_length.to_u32()) << 25;
        infos |= (self.r_extern as u32) * 0x0800_0000;
        infos |= (self.r_type as u32) << 28;
        write.write_u32_native(infos);
    }

    #[cfg(target_endian = "big")]
    pub fn write_into(self, write: &mut impl Write) {
        write.write_i32_native(self.r_address);

        let mut infos: u32 = 0;
        infos |= self.r_symbolnum << 8;
        infos |= (self.r_pcrel as u32) * 0x0000_0080;
        infos |= (self.r_length.to_u32()) << 5;
        infos |= (self.r_extern as u32) * 0x0000_0010;
        infos |= self.r_type as u32;
        write.write_u32_native(infos);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum RelocLength {
    /// 1 byte
    Byte = 0,
    /// 2 byte
    Word = 1,
    /// 4 byte
    Long = 2,
    /// 8 byte
    Quad = 3,
}

impl RelocLength {
    pub fn to_u32(self) -> u32 {
        self as u32
    }

    pub fn from_u32(n: u32) -> RelocLength {
        FromPrimitive::from_u32(n).unwrap()
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

        assert_eq!(buf.len(), RelocationInfo::SIZE as usize);

        let read_reloc = RelocationInfo::read_from_in(&mut buf.as_slice(), Endian::NATIVE);

        assert_eq!(read_reloc, reloc);
    }
}
