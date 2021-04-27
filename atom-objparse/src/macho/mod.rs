mod header;
mod load_command;
mod nlist;

use self::load_command::parse_load_command;
use crate::reader::Reader;
use atom_macho::{load_command::segment64::Section64, MachO};

pub fn parse_macho(buf: &mut Reader) -> Option<MachO> {
    let header = header::parse_macho_header(buf)?;

    let mut load_commands = Vec::with_capacity(header.n_cmds as usize);
    for _ in 0..header.n_cmds {
        load_commands.push(parse_load_command(buf));
    }

    Some(MachO {
        header,
        load_commands,
    })
}

pub fn parse_section(buf: &Reader, section: &Section64) -> Vec<u8> {
    let data_start = section.offset as usize;
    let data_end = data_start + section.size as usize;
    let data_slice = &buf.get_full_slice()[data_start..data_end];
    data_slice.to_vec()
}
