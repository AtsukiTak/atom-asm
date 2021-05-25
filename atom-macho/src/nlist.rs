use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NList64 {
    /// An index into the string table. To specify an empty string (""), set this value to 0.
    pub n_strx: u32,
    /// this field really contains four fields.
    pub n_type: NTypeField,
    /// If the type is NType::Sect then the n_sect field contains an ordinal of the section the
    /// symbol is defined in. The sections are numbered from 1 and refer to sections in order they
    /// appear in the load commands for the file they are in. This means the same ordinal may very
    /// well refer to different sections in different files.
    ///
    /// The n_value field for all symbol table entries (including n_stab's) gets updated by the
    /// link editor based on the value of its n_sect field and where the section n_sect references
    /// gets relocated. If the value of the n_sect field is NO_SECT then it's n_value field is not
    /// changed by the link editor.
    pub n_sect: u8,
    pub n_desc: u16,
    pub n_value: u64,
}

impl NList64 {
    pub const SIZE: u32 = 0x10; // 16

    pub const NO_SECT: u8 = 0;
    pub const MAX_SECT: u8 = 255;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NTypeField {
    Norm {
        /// private external symbol bit
        n_pext: bool,
        n_type: NType,
        /// set for external symbols
        n_ext: bool,
    },
    /// if any of n_stab bits on, this entry is a symbolic debugging entriy.
    Stab(DebugSymbol),
}

impl NTypeField {
    pub const N_STAB_MASK: u8 = 0xe0;
    pub const N_PEXT_MASK: u8 = 0x10;
    pub const N_TYPE_MASK: u8 = 0x0e;
    pub const N_EXT_MASK: u8 = 0x01;

    pub fn from_u8(n: u8) -> Self {
        let n_stab = n & Self::N_STAB_MASK;
        if n_stab == 0 {
            let n_pext = n & Self::N_PEXT_MASK == Self::N_PEXT_MASK;
            let n_type = NType::from_u8(n & Self::N_TYPE_MASK);
            let n_ext = n & Self::N_EXT_MASK == Self::N_EXT_MASK;
            NTypeField::Norm {
                n_pext,
                n_type,
                n_ext,
            }
        } else {
            NTypeField::Stab(DebugSymbol::from_u8(n_stab))
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            NTypeField::Norm {
                n_pext,
                n_type,
                n_ext,
            } => n_pext as u8 * Self::N_PEXT_MASK | n_type.to_u8() | n_ext as u8 * Self::N_EXT_MASK,
            NTypeField::Stab(stab) => stab.to_u8(),
        }
    }
}

#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NType {
    /// Undefined, n_sect == NO_SECT
    Undf = 0x0,
    /// Absolute, n_sect == NO_SECT
    Abs = 0x2,
    /// Defined in section number n_sect
    Sect = 0xe,
    /// Prebound undefined (defined in a dylib)
    Pbud = 0xc,
    /// Indirect.
    /// If the type is NType::Indr then the symbol is defined to be the same as another symbol. In
    /// this case the n_value field is an index into the string table of the other symbol's name.
    /// When the other symbol is defined then they both take on the defined type and value.
    Indr = 0xa,
}

impl NType {
    pub fn from_u8(n: u8) -> Self {
        FromPrimitive::from_u8(n).unwrap_or_else(|| panic!("Invalid n_type number {}", n))
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

/// TODO : implement all
#[derive(FromPrimitive, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugSymbol {
    /// global symbol
    Gsym = 0x20,
}

impl DebugSymbol {
    pub fn from_u8(n: u8) -> Self {
        FromPrimitive::from_u8(n).unwrap_or_else(|| panic!("Unsupported debug symbol {}", n))
    }

    pub fn to_u8(self) -> u8 {
        self as u8
    }
}
