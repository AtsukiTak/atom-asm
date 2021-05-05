mod header;
mod load_command;
mod nlist;

use self::{load_command::parse_load_command, nlist::parse_nlist};
use crate::{hex::Hex, reader::Reader};
use atom_macho::{
    header::Header64,
    load_command::{segment64::Section64, LoadCommand},
    nlist::NList64,
    string_table::StringTable,
};

#[derive(Debug)]
pub struct FullMacho {
    header: Header64,
    load_commands: Vec<LoadCommand>,
    sections: Vec<Hex<Vec<u8>>>,
    nlists: Vec<NList64>,
    string_table: Option<StringTable>,
}

pub fn parse_macho(buf: &mut Reader) -> Option<FullMacho> {
    let header = header::parse_macho_header(buf)?;

    let mut load_commands = Vec::with_capacity(header.n_cmds as usize);
    let mut sections = Vec::new();
    let mut nlists = Vec::new();
    let mut string_table = None;

    for _ in 0..header.n_cmds {
        let cmd = parse_load_command(buf);

        match &cmd {
            LoadCommand::Segment64(_, ref sects) => {
                let mut buf = buf.clone();
                for sect in sects {
                    buf.set_pos(sect.offset as usize);
                    let data = buf.read_bytes(sect.size as usize).to_vec();
                    sections.push(Hex::new(data));
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
                buf.set_pos(symtab.stroff as usize);
                let str_table_data = buf.read_bytes(symtab.strsize as usize);
                string_table.replace(StringTable::new(str_table_data.to_vec()));
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
        string_table,
    })
}
