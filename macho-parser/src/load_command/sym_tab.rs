use crate::Buffer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymTab {
    cmd_size: u32,
}

impl SymTab {
    pub fn parse(buf: &mut Buffer) -> Self {
        let cmd_size = buf.read_u32();

        buf.skip((cmd_size - 4 - 4) as usize);

        SymTab { cmd_size }
    }
}
