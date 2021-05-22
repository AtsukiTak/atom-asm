use atom_macho::load_command::symtab::SymtabCommand;

pub trait SymtabCommandConfigure {
    fn new() -> Self;
}

impl SymtabCommandConfigure for SymtabCommand {
    fn new() -> Self {
        SymtabCommand {
            cmd: SymtabCommand::TYPE,
            cmdsize: SymtabCommand::SIZE,
            symoff: 0,
            nsyms: 0,
            stroff: 0,
            strsize: 0,
        }
    }
}
