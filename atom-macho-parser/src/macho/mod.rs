mod header;
mod load_command;
mod nlist;

use self::load_command::parse_load_command;
use crate::buffer::Buffer;
use atom_macho_types::MachO;

pub fn parse_macho(buf: &mut Buffer) -> Option<MachO> {
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
