mod header;
mod load_command;
mod nlist;

use self::{load_command::parse_load_command, nlist::parse_nlist};
use crate::{hex::Hex, reader::Reader};
use atom_macho::{
    header::Header64,
    load_command::{segment64::Section64, LoadCommand},
    nlist::NList64,
};

#[derive(Debug)]
pub struct FullMacho {
    header: Header64,
    load_commands: Vec<LoadCommand>,
    sections: Vec<Hex<Vec<u8>>>,
    nlists: Vec<NList64>,
}

pub fn parse_macho(buf: &mut Reader) -> Option<FullMacho> {
    let header = header::parse_macho_header(buf)?;

    let mut load_commands = Vec::with_capacity(header.n_cmds as usize);
    let mut sections = Vec::new();
    let mut nlists = Vec::new();

    for _ in 0..header.n_cmds {
        let cmd = parse_load_command(buf);

        match &cmd {
            LoadCommand::Segment64(_, ref sects) => {
                for sect in sects {
                    sections.push(extract_section(buf, sect));
                }
            }
            LoadCommand::Symtab(symtab) => {
                let mut buf = buf.clone();

                // parse nlist
                buf.set_pos(symtab.symoff as usize);
                for _ in 0..symtab.nsyms {
                    nlists.push(parse_nlist(&mut buf));
                }

                // extract string table
            }
            _ => {}
        };

        load_commands.push(cmd);
    }

    Some(FullMacho {
        header,
        load_commands,
        sections,
        nlists,
    })
}

fn extract_section(buf: &Reader, section: &Section64) -> Hex<Vec<u8>> {
    let data_start = section.offset as usize;
    let data_end = data_start + section.size as usize;
    let data_slice = &buf.get_full_slice()[data_start..data_end];
    Hex::new(data_slice.to_vec())
}
