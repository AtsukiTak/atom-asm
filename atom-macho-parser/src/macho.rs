use crate::buffer::Buffer;
use atom_macho_types as macho;

pub fn parse_macho(buf: &mut Buffer) -> Option<macho::header::Magic> {
    let magic_n = buf.read_u32();
    let magic = macho::header::Magic::from_u32(magic_n)?;

    Some(magic)
}
