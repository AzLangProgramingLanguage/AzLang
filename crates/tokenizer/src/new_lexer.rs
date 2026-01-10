use std::{iter::Peekable, str::Chars};

use crate::{errors::LexerError, iterator::{SourceSpan, Tokens}, tokens::Token, words::tokenize_word};
pub struct NewLexer<'a> {
    chars: Peekable<Chars<'a>>,
    is_line_start: bool,
    indent_level: usize,
    space: usize,
    line: u32,
    start: u32,
    end: u32,
}
impl<'a> NewLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            is_line_start: false,
            indent_level: 0,
            space: 0,
            line: 0,
            start: 0,
            end: 0,
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.chars.peek() {
            if *ch == ' ' && self.is_line_start {
                self.space += 1;
                self.start += 1;
                self.end += 1;
                self.chars.next();
            } else if *ch == ' ' && !self.is_line_start {
                self.start += 1;
                self.end += 1;
                self.chars.next();
            } else {
                break;
            }
        }
    }
    pub fn tokenize(&mut self) -> Result<Tokens, LexerError> {
        let mut tokens = Tokens::default();

        loop {
            let token = self.next_token()?;
            match token {
                Token::Eof => break,
                _ => tokens.push(
                    token,
                    SourceSpan {
                        start: self.start,
                        end: self.end,
                        line: self.line,
                    },
                ),

            }
            self.start = self.end;
        }
        Ok(tokens)
    }
    fn read_string(&mut self) -> Result<Token, LexerError> {
        let mut str = String::new();
        for ch in &mut self.chars {
            match ch {
                '"' => break,
                '\n' => return Err(LexerError::UnClosedString(
                    SourceSpan {
                        start: self.start,
                        end: self.end, /* TODO: Test edilməli */
                        line: self.line,
                    },
                    str,
                )),
                _ => {
                    str.push(ch);
                }
            }
            self.end += 1;
        }

        Ok(Token::StringLiteral(str))
    }
    fn read_word(&mut self) -> Result<Token, LexerError> {
        let mut str = String::new();
        while let Some(ch) = self.chars.peek() {
            if ch.is_alphanumeric() {
                str.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }
        self.end = self.start + str.len() as u32;
        Ok(tokenize_word(str.as_str()))
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let mut buf = String::new();
        let mut has_dot = false;
        while let Some(ch) = self.chars.peek() {
            match ch {
                '0'..='9' => {
                    buf.push(*ch);
                    self.chars.next();
                }

                'A'..='Z' | 'a'..='z' => {
                    return Err(LexerError::NumberAndAlpha);
                }

                '.' if !has_dot => {
                    has_dot = true;
                    buf.push('.');
                    self.chars.next();
                }

                _ => break,
            }
        }

        self.end = self.start + buf.len() as u32;
       
        if has_dot {
            Ok(Token::Float(
                buf.parse::<f64>().map_err(LexerError::FloatUnKnow)?,
            ))
        } else {
            if buf.starts_with('0') && buf.len() > 1 {
                return Err(LexerError::CannotStartZeroNumber(SourceSpan {
                    start: self.start,
                    end: self.end,
                    line: self.line,
                }, buf));
            }
            Ok(Token::Number(
                buf.parse::<i64>().map_err(LexerError::NumberUnKnow)?,
            ))
        }
    }
    fn consume(&mut self, token: Token) -> Result<Token, LexerError> {
        self.chars.next();
        self.end += 1;
        Ok(token)
    }

    fn handle_indentation(&mut self) -> Result<Option<Token>, LexerError> {
        if let Some('\n') = self.chars.peek() {
            self.is_line_start = true;
            self.space = 0;
            self.chars.next();
            self.line += 1;
            self.start = 0;
            self.end = 0;
            return Ok(Some(Token::Newline));
        };
        if self.space == self.indent_level * 4 {
            Ok(None)
        } else if self.space == (self.indent_level + 1) * 4 {
            self.indent_level += 1;
            Ok(Some(Token::Indent))
        } else if self.space.is_multiple_of(4) && self.space < self.indent_level * 4 {
            self.indent_level -= 1;
            Ok(Some(Token::Dedent))
        } else {
            Err(LexerError::InCorrectSpaceSize)
        }
    }
    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        match self.handle_indentation() {
            Ok(Some(token)) => return Ok(token),
            Ok(None) => {}
            Err(e) => return Err(e),
        }

        let char = self.chars.peek();
        let token = match char {
            Some('(') => self.consume(Token::LParen),
            Some(')') => self.consume(Token::RParen),
            Some(':') => self.consume(Token::Colon),
            Some(',') => self.consume(Token::Comma),
            Some('{') => self.consume(Token::LBrace),
            Some('.') => self.consume(Token::Dot),
            Some('}') => self.consume(Token::RBrace),
            Some('_') => self.consume(Token::Underscore),
            Some('[') => self.consume(Token::ListStart),
            Some(']') => self.consume(Token::ListEnd),
            Some('=') => self.consume(Token::Op('=')),
            Some('/') => self.consume(Token::Op('/')),
            Some('+') => self.consume(Token::Op('+')),
            Some('-') => self.consume(Token::Op('-')),
            Some('*') => self.consume(Token::Op('*')),
            Some('0'..='9') => self.read_number(),
            Some('\'') | Some('"') => self.read_string(),
            Some(_) => self.read_word(),
            None => Ok(Token::Eof),
        };
        self.is_line_start = false;
        token
    }
}
