use std::fmt;

pub struct HexVec<'a>(&'a Vec<u8>);

impl<'a> HexVec<'a> {
    pub fn new(vec: &'a Vec<u8>) -> Self {
        HexVec(vec)
    }
}

impl<'a> fmt::Debug for HexVec<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_fmt(format_args!("{:02X?}", self.0))
    }
}
