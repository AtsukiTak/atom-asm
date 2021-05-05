use crate::reader::Reader;
use atom_macho::nlist::{NList64, NTypeField};

pub fn parse_nlist(buf: &mut Reader) -> NList64 {
    let n_strx = buf.read_u32();

    let n_type_n = buf.read_u8();
    let n_type = NTypeField::from_u8(n_type_n);

    let n_sect = buf.read_u8();
    let n_desc = buf.read_u16();
    let n_value = buf.read_u64();

    NList64 {
        n_strx,
        n_type,
        n_sect,
        n_desc,
        n_value,
    }
}
