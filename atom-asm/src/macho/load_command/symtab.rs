use atom_macho::load_command::symtab::SymtabCommand;

pub struct SymtabCommandBuilder {
    cmd: SymtabCommand,
}

impl SymtabCommandBuilder {
    pub fn new() -> Self {
        SymtabCommandBuilder {
            cmd: SymtabCommand {
                cmd: SymtabCommand::TYPE,
                cmdsize: SymtabCommand::SIZE,
                symoff: 0,
                nsyms: 0,
                stroff: 0,
                strsize: 0,
            },
        }
    }

    pub fn sym_off(&mut self, sym_off: u32) -> &mut Self {
        self.cmd.symoff = sym_off;
        self
    }

    pub fn n_syms(&mut self, n_syms: u32) -> &mut Self {
        self.cmd.nsyms = n_syms;
        self
    }

    pub fn str_off(&mut self, str_off: u32) -> &mut Self {
        self.cmd.stroff = str_off;
        self
    }

    pub fn str_size(&mut self, str_size: u32) -> &mut Self {
        self.cmd.strsize = str_size;
        self
    }

    pub fn build(&self) -> SymtabCommand {
        self.cmd.clone()
    }
}
