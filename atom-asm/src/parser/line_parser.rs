use super::line_stream::RawLine;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    SectionDeclare(SectionType),
    GlobalSymbolDef(String),
    SymbolDef(String),
    InstructionDef(Instruction),
    /*
    DataDef(Data),
    BssDef(Bss),
    ConstDef(Const),
    */
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SectionType {
    Text,
    Data,
    Bss,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    bytes: [u8; 15],
    len: u8,
}

pub fn parse_line(raw_line: RawLine) -> Option<Line> {
    let mut tokens = raw_line.tokens();

    let line = match raw_line.nth_token(0).map(|t| t.as_str()) {
        Some("section") => parse_section_declare(&raw_line),
        Some("global") => parse_global_symbol_def(&raw_line),
        Some(s) if s.ends_with(":") => parse_symbol_def(&raw_line),
        Some(s) => parse_instruction(&raw_line),
        None => return None,
    };

    Some(line)
}

fn parse_section_declare(line: &RawLine) -> Line {
    line.expect_token_num(2);

    let sect_type = match line.nth_token(1).map(|t| t.as_str()) {
        Some(".text") => SectionType::Text,
        Some(".data") => SectionType::Data,
        Some(".bss") => SectionType::Bss,
        Some(sect) => panic!("Unrecognized section {}", sect),
        None => panic!("section name is not specified"),
    };
    Line::SectionDeclare(sect_type)
}

fn parse_global_symbol_def(line: &RawLine) -> Line {
    line.expect_token_num(2);

    // グローバルシンボル定義
    let symbol_name = match line.nth_token(1) {
        Some(sym) => sym.as_str().to_string(),
        None => panic!("symbol name is not specified"),
    };
    Line::GlobalSymbolDef(symbol_name)
}

fn parse_symbol_def(line: &RawLine) -> Line {
    line.expect_token_num(1);

    // シンボル定義
    let token_str = line.nth_token(0).unwrap().as_str();
    let (symbol, _colon) = token_str.split_at(token_str.len() - 1);
    Line::SymbolDef(symbol.to_string())
}

fn parse_instruction(line: &RawLine) -> Line {
    match line.nth_token(0).unwrap().as_str() {
        "mov" => todo!(),
        "syscall" => todo!(),
        opcode => panic!("unrecognized opcode {}", opcode),
    }
}
