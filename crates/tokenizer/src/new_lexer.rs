use std::{collections::VecDeque, str::Chars};

use crate::tokens::Token;

pub struct NewSourceSpan {
    line: u32,
    start: u32,
}
pub struct NewLexer<'a> {
    chars: Chars<'a>,
}
impl<'a> NewLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars(),
        }
    }
    pub fn tokenize(&mut self) {
        let mut tokens: Vec<Token> = vec![];
        while let Some(token) = self.next_token() {
            println!("{token}")
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        match self.chars.next() {
            Some('\n') => Some(Token::Newline),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            Some(':') => Some(Token::Colon),
            Some(',') => Some(Token::Comma),
            Some('{') => Some(Token::LBrace),
            Some('}') => Some(Token::RBrace),
            Some('_') => Some(Token::Underscore),
            Some('[') => Some(Token::ListStart),
            Some(']') => Some(Token::ListEnd),
            Some(other) => {
                println!("Bilinmeyen Token Tapıldı token:{:?}", other);
                Some(Token::Void)
            }

            None => None,
        }
    }
}
