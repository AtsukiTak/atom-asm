use atom_macho::load_command::sym_tab::SymTab;

pub struct SymTabBuilder {
    cmd: SymTab,
}

impl SymTabBuilder {
    pub fn new() -> Self {
        SymTabBuilder {
            cmd: SymTab {
                cmd_size: SymTab::CMD_SIZE,
                sym_off: 0,
                n_syms: 0,
                str_off: 0,
                str_size: 0,
            },
        }
    }

    pub fn sym_off(&mut self, sym_off: u32) -> &mut Self {
        self.cmd.sym_off = sym_off;
        self
    }

    pub fn n_syms(&mut self, n_syms: u32) -> &mut Self {
        self.cmd.n_syms = n_syms;
        self
    }

    pub fn str_off(&mut self, str_off: u32) -> &mut Self {
        self.cmd.str_off = str_off;
        self
    }

    pub fn str_size(&mut self, str_size: u32) -> &mut Self {
        self.cmd.str_size = str_size;
        self
    }

    pub fn build(&self) -> SymTab {
        self.cmd.clone()
    }
}
