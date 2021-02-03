mod header;
mod macho;
mod magic;
mod parser;

pub use self::{
    header::{CpuType, Header},
    macho::MachO,
    magic::Magic,
    parser::parse,
};
