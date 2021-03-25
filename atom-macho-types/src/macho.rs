use crate::{Buffer, Header, LoadCommand};

#[derive(Debug, Clone)]
pub struct MachO {
    pub header: Header,
    pub load_commands: Vec<LoadCommand>,
}

impl MachO {
    pub fn parse(buf: &mut Buffer) -> Self {
        let header = Header::parse(buf);

        let mut load_commands = Vec::new();
        for _ in 0..header.n_cmds {
            let cmd = LoadCommand::parse(buf);
            load_commands.push(cmd);
        }

        MachO {
            header,
            load_commands,
        }
    }
}
