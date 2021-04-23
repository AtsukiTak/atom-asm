use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NList64 {
    /// An index into the string table. To specify an empty string (""), set this value to 0.
    pub n_strx: u32,
    pub n_type: NType,
    /// private external.
    pub n_pext: bool,
    /// external.
    pub n_ext: bool,
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u64,
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NType {
    UNDF = 0b0000_0000,
    ABS = 0b0000_0010,
    SECT = 0b0000_1110,
    PBUD = 0b0000_1100,
    INDR = 0b0000_1010,
}

impl NType {
    pub fn from_u8(n: u8) -> Self {
        FromPrimitive::from_u8(n).expect(format!("Invalid n_type number {}", n).as_str())
    }
}
