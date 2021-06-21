pub enum Symbol {
    Undef { name: String },
    Abs { name: String, val: u64, ext: bool },
    Ref { name: String, addr: u64, ext: bool },
}

impl Symbol {
    pub fn name(&self) -> &str {
        match self {
            Symbol::Undef { name } => name.as_str(),
            Symbol::Abs { name, .. } => name.as_str(),
            Symbol::Ref { name, .. } => name.as_str(),
        }
    }

    pub fn ext(&self) -> bool {
        match self {
            Symbol::Undef { .. } => true,
            Symbol::Abs { ext, .. } => *ext,
            Symbol::Ref { ext, .. } => *ext,
        }
    }
}
