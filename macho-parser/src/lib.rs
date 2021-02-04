mod header;
mod macho;
mod magic;
mod parser;

pub use self::{header::*, macho::MachO, magic::Magic, parser::parse};
