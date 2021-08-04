use std::io::{BufRead, BufReader, Read};

pub struct LineStream<R> {
    read: BufReader<R>,
    next_line_num: usize,
}

impl<R: Read> LineStream<R> {
    pub fn new(read: R) -> Self {
        LineStream {
            read: BufReader::new(read),
            next_line_num: 0,
        }
    }
}

impl<R: Read> Iterator for LineStream<R> {
    type Item = RawLine;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_line_num += 1;

        let mut raw_str = String::new();

        // 次の行を読み込み
        let is_eof = self.read.read_line(&mut raw_str).unwrap() == 0;
        if is_eof {
            return None;
        }

        Some(RawLine {
            raw_str,
            line_num: self.next_line_num,
        })
    }
}

pub struct RawLine {
    raw_str: String,
    line_num: usize,
}

impl RawLine {
    pub fn tokens<'a>(&'a self) -> impl Iterator<Item = Token<'a>> {
        // コメントを取り除く
        let s_uncommented = match self.raw_str.find(";") {
            None => self.raw_str.as_str(),
            Some(i) => self.raw_str.split_at(i).0,
        };

        s_uncommented
            .split(|c: char| c.is_whitespace() || c == ',')
            .map(|s| Token { s })
    }

    pub fn nth_token<'a>(&'a self, idx: usize) -> Option<Token<'a>> {
        self.tokens().nth(idx)
    }

    pub fn expect_token_num(&self, n: usize) {
        // TODO
        // もっと丁寧なエラーメッセージをだす
        assert_eq!(self.tokens().count(), n);
    }
}

pub struct Token<'a> {
    s: &'a str,
    // column_num とかを保持したい
}

impl<'a> Token<'a> {
    pub fn as_str(&self) -> &'a str {
        self.s
    }
}
