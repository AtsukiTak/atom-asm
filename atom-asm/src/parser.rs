use std::io::{BufRead, Error as IoError};

pub struct LineStream<R> {
    read: R,
    buf: String,
}

impl<R: BufRead> LineStream<R> {
    pub fn new(read: R) -> Self {
        LineStream {
            read,
            buf: String::new(),
        }
    }
}

impl<R: BufRead> Iterator for LineStream<R> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.clear();

        // 次の行を読み込み
        let is_eof = self.read.read_line(&mut self.buf).unwrap() == 0;
        if is_eof {
            return None;
        }

        match parse_line(self.buf.as_str()) {
            Some(line) => Some(line),
            // 空行orコメント行
            None => self.next(),
        }
    }
}

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

fn parse_line(s: &str) -> Option<Line> {
    assert!(s.ends_with("\n"));

    // コメントを取り除く
    let s_uncommented = match s.find(";") {
        None => s,
        Some(i) => s.split_at(i).0,
    };

    let mut tokens = s_uncommented.split(|c: char| c.is_whitespace() || c == ',');

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
    Some(Line::Content(s_uncommented.to_string()))
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
