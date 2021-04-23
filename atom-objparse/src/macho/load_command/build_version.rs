use crate::buffer::Buffer;
use atom_macho::load_command::build_version::{
    BuildToolVersion, BuildVersion, Platform, Tool, Version,
};

pub fn parse_build_version(buf: &mut Buffer) -> BuildVersion {
    let cmd_type = buf.read_u32();
    if cmd_type != BuildVersion::CMD_TYPE {
        panic!("Invalid cmd number");
    }

    let cmd_size = buf.read_u32();
    let platform = Platform::from_u32(buf.read_u32());
    let minos = Version::from_u32(buf.read_u32());
    let sdk = Version::from_u32(buf.read_u32());
    let ntools = buf.read_u32();

    let mut tools = Vec::with_capacity(ntools as usize);
    for _ in 0..ntools {
        tools.push(BuildToolVersion {
            tool: Tool::from_u32(buf.read_u32()),
            version: buf.read_u32(),
        });
    }

    // 必ず8バイト境界にそろうためアライメントは不要

    BuildVersion {
        cmd_size,
        platform,
        minos,
        sdk,
        ntools,
        tools,
    }
}
