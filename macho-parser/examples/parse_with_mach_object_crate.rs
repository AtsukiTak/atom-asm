use mach_object::{LoadCommand, MachCommand, OFile, CPU_TYPE_X86_64};
use std::io::Cursor;

fn main() {
    let bytes = include_bytes!("./a.out");
    let mut cur = Cursor::new(bytes);
    if let OFile::MachFile {
        ref header,
        ref commands,
    } = OFile::parse(&mut cur).unwrap()
    {
        dbg!(header);
        dbg!(commands);
        assert_eq!(header.cputype, CPU_TYPE_X86_64);
        assert_eq!(header.ncmds as usize, commands.len());
        for &MachCommand(ref cmd, _cmdsize) in commands {
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
