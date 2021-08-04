use std::io::{BufRead, BufReader, Read};

pub struct LineStream<R> {
    read: BufReader<R>,
    buf: String,
    next_line_num: usize,
}

impl<R: Read> LineStream<R> {
    pub fn new(read: R) -> Self {
        LineStream {
            read: BufReader::new(read),
            buf: String::new(),
            next_line_num: 0,
        }
    }

    fn next<'a>(&'a mut self) -> Option<RawLine<'a>> {
        self.next_line_num += 1;
        self.buf.clear();

        // 次の行を読み込み
        let is_eof = self.read.read_line(&mut self.buf).unwrap() == 0;
        if is_eof {
            return None;
        }

        Some(RawLine {
            raw_str: self.buf.as_str(),
            line_num: self.next_line_num,
        })
    }
}

pub struct RawLine<'a> {
    raw_str: &'a str,
    line_num: usize,
}

impl<'a> RawLine<'a> {
    pub fn tokens(&self) -> impl Iterator<Item = Token<'a>> {
        // コメントを取り除く
        let s_uncommented = match self.raw_str.find(";") {
            None => self.raw_str,
            Some(i) => self.raw_str.split_at(i).0,
        };

        s_uncommented
            .split(|c: char| c.is_whitespace() || c == ',')
            .map(|s| Token { s })
    }

    pub fn nth_token(&'a self, idx: usize) -> Option<Token<'a>> {
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
