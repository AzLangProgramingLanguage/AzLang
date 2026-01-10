use std::collections::VecDeque;
use std::fmt;
use std::fmt::Display;

use crate::tokens::Token;

#[derive(Debug)]
pub struct SourceSpan {
    pub start: u32,
    pub end: u32,
    pub line: u32,
}
impl Display for SourceSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sətir {}, sütun {}", self.line, self.start)
    }
}
#[derive(Debug)]
pub struct SpannedToken {
    pub token: Token,
    pub span: SourceSpan,
}
#[derive(Default,Debug)]
pub struct Tokens {
    source: VecDeque<SpannedToken>,
}

impl Iterator for Tokens {
    type Item = SpannedToken;
    fn next(&mut self) -> Option<Self::Item> {
        self.source.pop_front()
    }
}

impl Tokens {
    pub fn push(&mut self, token: Token, span: SourceSpan) {
        self.source.push_back(SpannedToken { token, span });
    }
    pub fn peek(&self) -> Option<&SpannedToken> {
        self.source.front()
    }
    pub fn peek_nth(&self, index: usize) -> Option<&SpannedToken> {
        self.source.get(index)
    }
}
