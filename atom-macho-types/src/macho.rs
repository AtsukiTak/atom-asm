use crate::{Header, LoadCommand, ReadBuf, WriteBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MachO {
    pub header: Header,
    pub load_commands: Vec<LoadCommand>,
}

impl MachO {
    pub fn parse(buf: &mut ReadBuf) -> Self {
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

    pub fn write(&self, buf: &mut WriteBuf) {
        self.header.write(buf);
        for cmd in self.load_commands.iter() {
            cmd.write(buf);
        }

        todo!()
    }
}
