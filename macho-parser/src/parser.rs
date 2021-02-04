use crate::{CpuType, FileType, Header, MachO, Magic};
use byteorder::{ByteOrder, NativeEndian, ReadBytesExt};
use num_traits::FromPrimitive as _;

#[cfg(target_endian = "little")]
type ReverseEndian = byteorder::BigEndian;

#[cfg(target_endian = "big")]
type ReverseEndian = byteorder::LittleEndian;

pub fn parse<Buf: ReadBytesExt>(buf: &mut Buf) -> MachO {
    let magic_n = buf
        .read_u32::<NativeEndian>()
        .expect("Unable to read magic number");
    let magic = Magic::from_u32(magic_n).expect("Invalid magic number");

    match magic {
        Magic::Magic64 => parse_inner::<Buf, NativeEndian>(buf, magic),
        Magic::Cigam64 => parse_inner::<Buf, ReverseEndian>(buf, magic),
        _ => unimplemented!(),
    }
}

fn parse_inner<Buf: ReadBytesExt, Endian: ByteOrder>(buf: &mut Buf, magic: Magic) -> MachO {
    let cpu_type_n = buf.read_i32::<Endian>().expect("Unable to read cpu_type");
    let cpu_subtype_n = buf
        .read_i32::<Endian>()
        .expect("Unable to read cpu_subtype");
    let cpu_type = CpuType::from_i32_i32(cpu_type_n, cpu_subtype_n);

    let file_type_n = buf.read_u32::<Endian>().expect("Unable to read file_type");
    let file_type = FileType::from_u32(file_type_n).expect("Invalid file_type number");

    let header = Header {
        magic,
        cpu_type,
        file_type,
    };

    MachO { header }
}
