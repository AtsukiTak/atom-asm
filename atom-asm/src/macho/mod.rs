mod header;
mod load_command;

use atom_macho::MachO;

pub fn gen_demo() -> MachO {
    let mut header = header::gen_x86_64();
    header.n_cmds = 2;
    header.size_of_cmds = 176;

    todo!()
}

pub fn serialize(macho: &MachO) -> Vec<u8> {
    todo!()
}
