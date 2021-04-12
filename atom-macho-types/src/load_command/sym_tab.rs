use crate::ReadBuf;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymTab {
    pub cmd_size: u32,
    pub sym_off: u32,
    pub n_syms: u32,
    /// An integer containing the byte offset from the start of the image to the location of the string table.
    pub str_off: u32,
    /// the size (in bytes) of the string table.
    pub str_size: u32,

    pub nlists: Vec<NList64>,
}

impl SymTab {
    pub fn parse(buf: &mut ReadBuf) -> Self {
        let cmd_size = buf.read_u32();
        let sym_off = buf.read_u32();
        let n_syms = buf.read_u32();
        let str_off = buf.read_u32();
        let str_size = buf.read_u32();

        // string table
        let mut s_table_buf = buf.clone();
        s_table_buf.set_pos(str_off as usize);

        let mut nlist_buf = buf.clone();
        nlist_buf.set_pos(sym_off as usize);
        let mut nlists = Vec::with_capacity(n_syms as usize);
        for _ in 0..n_syms {
            nlists.push(NList64::parse(&mut nlist_buf, &mut s_table_buf.clone()));
        }

        SymTab {
            cmd_size,
            sym_off,
            n_syms,
            str_off,
            str_size,
            nlists,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NList64 {
    /// An index into the string table. To specify an empty string (""), set this value to 0.
    n_strx: u32,
    n_type: NType,
    /// private external.
    n_pext: bool,
    /// external.
    n_ext: bool,
    n_sect: u8,
    n_desc: u16,
    n_value: u64,

    string: String,
}

impl NList64 {
    pub fn parse(buf: &mut ReadBuf, string_table: &mut ReadBuf) -> Self {
        let n_strx = buf.read_u32();
        let string = string_table.skip(n_strx as usize).read_c_string();

        let n_type_n = buf.read_u8();
        let n_stab = n_type_n & 0b1110_0000;
        if n_stab != 0 {
            // interpret n_type field as a stab value. See "mach-o/stab.h".
            unimplemented!();
        }
        let n_pext = n_type_n & 0b0001_0000 == 0b0001_0000;
        let n_type = NType::from_u8(n_type_n & 0b0000_1110);
        let n_ext = n_type_n & 0b0000_0001 == 0b0000_0001;

        let n_sect = buf.read_u8();
        let n_desc = buf.read_u16();
        let n_value = buf.read_u64();

        NList64 {
            n_strx,
            n_type,
            n_pext,
            n_ext,
            n_sect,
            n_desc,
            n_value,
            string,
        }
    }
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
    fn from_u8(n: u8) -> Self {
        FromPrimitive::from_u8(n).expect(format!("Invalid n_type number {}", n).as_str())
    }
}
