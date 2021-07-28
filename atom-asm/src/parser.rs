use std::io::{BufRead, Error as IoError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    SectionDeclare(SectionType),
    GlobalSymbol(String),
    SymbolDef(String),
    Content(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SectionType {
    Text,
    Data,
    Bss,
}

fn parse<R: BufRead>(reader: &mut R) -> Result<Vec<Line>, IoError> {
    let mut buf = String::new();
    let mut lines = Vec::new();

    while reader.read_line(&mut buf)? != 0 {
        if let Some(stripped_line) = StrippedLine::new(buf.as_str()) {
            if let Some(parsed_line) = stripped_line.parse_line() {
                lines.push(parsed_line);
            }
        }
        buf.clear();
    }

    Ok(lines)
}

// コメントを除いたアセンブリ行
struct StrippedLine<'a>(&'a str);

impl<'a> StrippedLine<'a> {
    fn new(raw: &'a str) -> Option<Self> {
        match raw.find(";") {
            None => Some(StrippedLine(raw)),
            Some(0) => None,
            Some(i) => Some(StrippedLine(raw.split_at(i).0)),
        }
    }

    fn tokens(&self) -> impl Iterator<Item = &'a str> {
        self.0.split_ascii_whitespace()
    }

    fn parse_line(&self) -> Option<Line> {
        let mut tokens = self.tokens();

        let token1 = match tokens.next() {
            Some(t) => t,
            None => return None,
        };

        // セクションの宣言
        if token1 == "section" {
            let sect_type = match tokens.next() {
                Some(".text") => SectionType::Text,
                Some(".data") => SectionType::Data,
                Some(".bss") => SectionType::Bss,
                Some(sect) => panic!("Unrecognized section {}", sect),
                None => panic!("section name is not specified"),
            };
            tokens.expect_end();
            return Some(Line::SectionDeclare(sect_type));
        }

        // グローバルシンボル定義
        if token1 == "global" {
            let symbol_name = match tokens.next() {
                Some(sym) => sym.to_string(),
                None => panic!("symbol name is not specified"),
            };
            tokens.expect_end();
            return Some(Line::GlobalSymbol(symbol_name));
        }

        // シンボル定義
        if token1.ends_with(":") {
            tokens.expect_end();
            let (symbol, _colon) = token1.split_at(token1.len() - 1);
            return Some(Line::SymbolDef(symbol.to_string()));
        }

        // コメント
        if token1.starts_with(";") {
            return None;
        }

        // 命令 or データ定義
        Some(Line::Content(self.0.to_string()))
    }
}

trait TokenIter<'a>: Iterator<Item = &'a str> {
    fn expect_end(&mut self) {
        match self.next() {
            Some(s) => {
                panic!("unexpected token : {}", s)
            }
            _ => {}
        }
    }
}

impl<'a, I> TokenIter<'a> for I where I: Iterator<Item = &'a str> {}
