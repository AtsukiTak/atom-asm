mod header;
mod load_command;

use self::load_command::{
    segment64::{Section64Builder, SegmentCommand64Builder},
    symtab::SymTabBuilder,
};
use atom_macho::MachO;

pub fn gen_demo() -> MachO {
    let mut header = header::gen_x86_64();
    header.n_cmds = 2;
    header.size_of_cmds = 176;

    let sect_data = vec![0x66, 0xB8, 0x2A, 0xC3];

    let sect_cmd = Section64Builder::new()
        .text_section()
        .addr(0)
        .size(sect_data.len() as u64)
        .offset(0xD0)
        .build();

    let seg_cmd = SegmentCommand64Builder::new()
        .add_section(sect_data.len() as u64)
        .fileoff(0xD0)
        .flags(0)
        .build();

    let symtab_cmd = SymTabBuilder::new()
        .sym_off(216)
        .n_syms(1)
        .str_off(232)
        .str_size(7)
        .build();

    todo!()
}

pub fn serialize(macho: &MachO) -> Vec<u8> {
    todo!()
}
