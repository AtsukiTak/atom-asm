use crate::reader::Reader;
use atom_macho::nlist::{NList64, NType};

pub fn parse_nlist(buf: &mut Reader) -> NList64 {
    let n_strx = buf.read_u32();
    let n_type_n = buf.read_u8();
    let n_stab = n_type_n & 0b1110_0000;
    if n_stab != 0 {
        // interpret n_type field as a stab value. See "mach-o/stab.h".
        unimplemented!();
    }
    let n_pext = n_type_n & 0b0001_0000 == 0b0001_0000;
    let n_type = NType::from_u8(n_type_n & 0b0000_1110);
    let n_ext = n_type_n & 0b0000_0001 == 0b0000_0001;

    let n_sect = buf.read_u8();
    let n_desc = buf.read_u16();
    let n_value = buf.read_u64();

    NList64 {
        n_strx,
        n_type,
        n_pext,
        n_ext,
        n_sect,
        n_desc,
        n_value,
    }
}
