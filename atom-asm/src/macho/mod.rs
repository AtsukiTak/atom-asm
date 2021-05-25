mod header;
mod load_command;
mod write;

pub use self::write::Write;

use self::{
    header::Header64Configure as _,
    load_command::{
        segment64::{Section64Configure as _, SegmentCommand64Configure as _},
        symtab::SymtabCommandConfigure as _,
    },
};
use crate::num::NumExt;
use atom_macho::{
    header::Header64,
    load_command::{
        segment64::{Section64, SegmentCommand64},
        symtab::SymtabCommand,
    },
    nlist::{NList64, NType, NTypeField},
    string_table::StringTable,
};

/// # Example Mach-O file format
///
///     00              08             0F
///     _________________________________
///  00 |            Header64           |
///  10 |_______________________________|
///  20 |                               |
///  30 |        Segment64Command       |
///  40 |                               |
///  50 |               ________________|
///  60 |_______________|               |
///  70 |                               |
///  80 |           Section64           |
///  90 |                               |
///  A0 |               ________________|
///  B0 |_______________|               |
///  C0 |_________SymtabCommand_________|
///  D0 |__SectionData__|__SymbolTable__|
///  E0 |__SymbolTable__|_StringTable_|
#[derive(Debug)]
pub struct MachO {
    pub header: Header64,
    pub segment_cmd: (SegmentCommand64, Vec<Section64>),
    pub symtab_cmd: SymtabCommand,
    pub sect_datas: Vec<Vec<u8>>,
    pub nlists: Vec<NList64>,
    pub string_table: StringTable,
}

impl MachO {
    pub fn new() -> Self {
        let mut header = Header64::new_x86_64();
        header.n_cmds = 2;
        header.size_of_cmds = SegmentCommand64::SIZE + SymtabCommand::SIZE;

        // SegmentCommand64の初期設定
        let mut segment_cmd = SegmentCommand64::new();
        // セクションデータはロードコマンドの直後にくる
        segment_cmd.fileoff =
            (Header64::SIZE + SegmentCommand64::SIZE + SymtabCommand::SIZE) as u64;

        // SymtabCommandの初期設定
        let mut symtab_cmd = SymtabCommand::new();
        // シンボルテーブルはセクションデータの直後にくる.
        // 今のところセクションデータの大きさは0
        symtab_cmd.symoff = segment_cmd.fileoff as u32;
        symtab_cmd.stroff = segment_cmd.fileoff as u32;
        symtab_cmd.strsize = 1; // 空文字

        // ストリングテーブルの初期化
        let mut string_table = StringTable::new();
        string_table.push_null();

        MachO {
            header,
            segment_cmd: (segment_cmd, Vec::new()),
            symtab_cmd: symtab_cmd,
            sect_datas: Vec::new(),
            nlists: Vec::new(),
            string_table,
        }
    }

    /// 新しいセクションデータを追加する
    /// 追加セクションの番号を返す
    pub fn add_text_section(&mut self, data: Vec<u8>) -> u8 {
        let (ref mut segment, ref mut sections) = self.segment_cmd;

        // 新しくSection64構造体が入る分、
        // - Header.size_of_cmds
        // - SegmentCommand64.cmdsize
        // - SegmentCommand64.fileoff
        // - Section64.offset
        // - SymtabCommand.symoff
        // - SymtabCommand.stroff
        // がズレる
        self.header.size_of_cmds += Section64::SIZE;
        segment.cmdsize += Section64::SIZE;
        segment.fileoff += Section64::SIZE as u64;
        for sect in sections.iter_mut() {
            sect.offset += Section64::SIZE;
        }
        self.symtab_cmd.symoff += Section64::SIZE;
        self.symtab_cmd.stroff += Section64::SIZE;

        // 新しくセクションデータが入る分、
        // - SymtabCommand.symoff
        // - SymtabCommand.stroff
        // がズレる
        // alignmentを含めたセクションデータのサイズ
        let sect_data_size = (data.len() as u32).aligned(8);
        self.symtab_cmd.symoff += sect_data_size;
        self.symtab_cmd.stroff += sect_data_size;

        // 新しく追加するSection64構造体
        let mut section = Section64::new();
        section.text_section();
        section.addr = 0;
        section.size = data.len() as u64;
        section.offset = Header64::SIZE
            + self.header.size_of_cmds
            + sections
                .iter()
                .map(|sect| sect.size.aligned(8) as u32)
                .sum::<u32>();
        sections.push(section);

        // セクションのサイズ
        segment.vmsize += data.len() as u64;
        segment.filesize += data.len() as u64;

        // データの追加
        self.sect_datas.push(data);
        segment.nsects += 1;

        // 追加されたセクションの番号（1始まり）
        self.sect_datas.len() as u8
    }

    /// 新しいシンボルを追加する
    pub fn add_symbol(&mut self, s: &str, section: u8, value: u64, external: bool) {
        self.symtab_cmd.nsyms += 1;
        // 文字列 + 空文字の大きさ
        self.symtab_cmd.strsize += s.len() as u32 + 1;

        // 新しくNList64が入る分、
        // - SymtabCommand.stroff
        // がズレる
        self.symtab_cmd.stroff += NList64::SIZE;

        let nlist = NList64 {
            n_strx: self.string_table.len() as u32,
            n_type: NTypeField::Norm {
                n_pext: false,
                n_type: NType::Sect,
                n_ext: external,
            },
            n_sect: section,
            n_desc: 0,
            n_value: value,
        };
        self.nlists.push(nlist);

        self.string_table.push(s);
        self.string_table.push_null();
    }
}
