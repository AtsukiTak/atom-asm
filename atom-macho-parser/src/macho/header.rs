use crate::buffer::Buffer;
use atom_macho_types::header::{CpuType, FileType, Flags, Header64, Magic};

pub fn parse_macho_header(buf: &mut Buffer) -> Option<Header64> {
    let magic_n = buf.read_u32();
    let magic = Magic::from_u32(magic_n)?;

    match magic {
        Magic::Magic64 => {}
        Magic::Cigam64 => {
            buf.set_reverse_endian();
        }
        _ => {
            panic!("Unsupported Mach-O file");
        }
    }

    let cpu_type_n = buf.read_i32();
    let cpu_subtype_n = buf.read_i32();
    let cpu_type = CpuType::from_i32_i32(cpu_type_n, cpu_subtype_n).expect("unsupported cpu type");

    let file_type_n = buf.read_u32();
    let file_type = FileType::from_u32(file_type_n).expect("unsupported file type");

    let n_cmds = buf.read_u32();

    let size_of_cmds = buf.read_u32();

    let flags_n = buf.read_u32();
    let flags = Flags::from_u32(flags_n).expect("unsupported flag");

    let reserved = buf.read_u32();

    Some(Header64 {
        magic,
        cpu_type,
        file_type,
        n_cmds,
        size_of_cmds,
        flags,
        reserved,
    })
}
