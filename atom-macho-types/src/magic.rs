use crate::Buffer;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

/// An integer containing a value identifying this file as a Mach-O file.
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Magic {
    /// use if the file is intended for use on a 64bit CPU with the **same** endianness as the host
    /// computer.
    Magic64 = 0xfeedfacf,
    /// use if the file is intended for use on a 64bit CPU with the **reverse** endianness as the
    /// host computer.
    Cigam64 = 0xcffaedfe,
    /// use if the file is intended for use on a 32bit CPU with the **same** endianness as the host
    /// computer.
    Magic = 0xfeedface,
    /// use if the file is intended for use on a 32bit CPU with the **reverse** endianness as the
    /// host computer.
    Cigam = 0xcefaedfe,
    /// use if the file contains code for more than one architecture and is intended for use on a
    /// CPU with the **same** endianness as the host computer.
    FatMagic = 0xcafebabe,
    /// use if the file contains code for more than one architecture and is intended for use on a
    /// CPU with the **reverse** endianness as the host computer.
    FatCigam = 0xbebafeca,
}

impl Magic {
    pub fn is_64bit(&self) -> bool {
        matches!(self, Magic::Magic64 | Magic::Cigam64)
    }

    pub fn parse(buf: &mut Buffer) -> Magic {
        assert!(buf.is_native_endian());

        let magic_n = buf.read_u32();
        let magic = Magic::from_u32(magic_n).expect("Invalid magic number");

        match magic {
            Magic::Cigam64 | Magic::Cigam | Magic::FatCigam => {
                buf.set_reverse_endian();
            }
            _ => {}
        }

        magic
    }
}
