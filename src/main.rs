use mach_object::{LoadCommand, MachCommand, OFile, CPU_TYPE_X86_64};
use std::fs::File;
use std::io::{Cursor, Read};

fn main() {
    let mut f = File::open("a.out").unwrap();
    let mut buf = Vec::new();
    let size = f.read_to_end(&mut buf).unwrap();
    let mut cur = Cursor::new(&buf[..size]);
    if let OFile::MachFile {
        ref header,
        ref commands,
    } = OFile::parse(&mut cur).unwrap()
    {
        dbg!(header);
        dbg!(commands);
        assert_eq!(header.cputype, CPU_TYPE_X86_64);
        assert_eq!(header.ncmds as usize, commands.len());
        for &MachCommand(ref cmd, cmdsize) in commands {
            if let &LoadCommand::Segment64 {
                ref segname,
                ref sections,
                ..
            } = cmd
            {
                println!("segment: {}", segname);
                for ref sect in sections {
                    println!("  section: {}", sect.sectname);
                }
            }
        }
    }
}
