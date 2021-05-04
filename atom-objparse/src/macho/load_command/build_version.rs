use crate::reader::Reader;
use atom_macho::load_command::build_version::{
    BuildToolVersion, BuildVersionCommand, Platform, Tool, Version,
};

pub fn parse_build_version(buf: &mut Reader) -> (BuildVersionCommand, Vec<BuildToolVersion>) {
    let cmd = buf.read_u32();
    if cmd != BuildVersionCommand::TYPE {
        panic!("Invalid cmd number");
    }

    let cmdsize = buf.read_u32();
    let platform = Platform::from_u32(buf.read_u32());
    let minos = Version::from_u32(buf.read_u32());
    let sdk = Version::from_u32(buf.read_u32());
    let ntools = buf.read_u32();

    let build_ver = BuildVersionCommand {
        cmd,
        cmdsize,
        platform,
        minos,
        sdk,
        ntools,
    };

    let mut tools = Vec::with_capacity(ntools as usize);
    for _ in 0..ntools {
        tools.push(BuildToolVersion {
            tool: Tool::from_u32(buf.read_u32()),
            version: buf.read_u32(),
        });
    }

    // 必ず8バイト境界にそろうためアライメントは不要

    (build_ver, tools)
}
