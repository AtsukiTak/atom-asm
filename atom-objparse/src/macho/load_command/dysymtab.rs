use crate::reader::Reader;
use atom_macho::load_command::DysymtabCommand;

pub fn parse_dysymtab(buf: &mut Reader) -> DysymtabCommand {
    let cmd = buf.read_u32();
    if cmd != DysymtabCommand::TYPE {
        panic!("Invalid cmd number");
    }

    let cmdsize = buf.read_u32();
    let ilocalsym = buf.read_u32();
    let nlocalsym = buf.read_u32();
    let iextdefsym = buf.read_u32();
    let nextdefsym = buf.read_u32();
    let iundefsym = buf.read_u32();
    let nundefsym = buf.read_u32();
    let tocoff = buf.read_u32();
    let ntoc = buf.read_u32();
    let modtaboff = buf.read_u32();
    let nmodtab = buf.read_u32();
    let extrefsymoff = buf.read_u32();
    let nextrefsyms = buf.read_u32();
    let indirectsymoff = buf.read_u32();
    let nindirectsyms = buf.read_u32();
    let extreloff = buf.read_u32();
    let nextrel = buf.read_u32();
    let locreloff = buf.read_u32();
    let nlocrel = buf.read_u32();

    DysymtabCommand {
        cmd,
        cmdsize,
        ilocalsym,
        nlocalsym,
        iextdefsym,
        nextdefsym,
        iundefsym,
        nundefsym,
        tocoff,
        ntoc,
        modtaboff,
        nmodtab,
        extrefsymoff,
        nextrefsyms,
        indirectsymoff,
        nindirectsyms,
        extreloff,
        nextrel,
        locreloff,
        nlocrel,
    }
}