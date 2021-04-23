use atom_macho_types::{
    header::{CpuSubTypeX86_64, CpuType, FileType, Flag, Flags, Header, Magic},
    MachO, ReadBuf,
};

#[test]
fn parse_and_check_sample_macho() {
    let bytes = include_bytes!("sample.o").to_vec();
    let mut buf = ReadBuf::new(bytes);

    let macho = MachO::parse(&mut buf);

    dbg!(&macho);

    assert_eq!(macho.header, expected_header());
}

fn expected_header() -> Header {
    let mut flags = Flags::new();
    flags.push(Flag::SubsectionsViaSymbols);

    Header {
        magic: Magic::Magic64,
        cpu_type: CpuType::X86_64(CpuSubTypeX86_64::All),
        file_type: FileType::Object,
        n_cmds: 4,
        size_of_cmds: 440,
        flags,
    }
}
