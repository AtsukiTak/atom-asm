use crate::{CpuType, FileType, Flag, Header, MachO, Magic};
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

    let header = match magic {
        Magic::Magic64 => parse_header::<Buf, NativeEndian>(buf, magic),
        Magic::Cigam64 => parse_header::<Buf, ReverseEndian>(buf, magic),
        _ => unimplemented!(),
    };

    MachO { header }
}

fn parse_header<Buf: ReadBytesExt, Endian: ByteOrder>(buf: &mut Buf, magic: Magic) -> Header {
    let cpu_type_n = buf.read_i32::<Endian>().expect("Unable to read cpu_type");
    let cpu_subtype_n = buf
        .read_i32::<Endian>()
        .expect("Unable to read cpu_subtype");
    let cpu_type = CpuType::from_i32_i32(cpu_type_n, cpu_subtype_n);

    let file_type_n = buf.read_u32::<Endian>().expect("Unable to read file_type");
    let file_type = FileType::from_u32(file_type_n).expect("Invalid file_type number");

    let n_cmds = buf.read_u32::<Endian>().expect("Unable to read ncmds");
    let size_of_cmds = buf
        .read_u32::<Endian>()
        .expect("Unable to read size_of_cmds");

    let flags_int = buf.read_u32::<Endian>().expect("Unable to read flags");
    let flags = Flag::vec_from_u32(flags_int);

    Header {
        magic,
        cpu_type,
        file_type,
        n_cmds,
        size_of_cmds,
        flags,
    }
}
