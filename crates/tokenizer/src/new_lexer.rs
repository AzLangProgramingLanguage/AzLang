use crate::errors::LexerError;
use crate::tokenize_word;
use crate::tokens::Token;
use std::{iter::Peekable, str::Chars};

pub struct NewSourceSpan {
    line: u32,
    start: u32,
}
pub struct NewLexer<'a> {
    chars: Peekable<Chars<'a>>,
    is_line_start: bool,
    indent_stack: Vec<usize>,
    pending_dedents: usize,
    pending_intend: usize,
    current_indent: usize,
}
impl<'a> NewLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            is_line_start: false,
            indent_stack: vec![],
            pending_dedents: 0,
            pending_intend: 0,
            current_indent: 0,
        }
    }
    fn skip_whitespace(&mut self) {
        println!("is white space isLineStart: {} peek: {:?}", self.is_line_start, self.chars.peek());
        while let Some(ch) = self.chars.peek() {
            if *ch == ' ' && self.is_line_start {
                self.current_indent += 1;
                self.chars.next();
            } else if *ch == ' ' && !self.is_line_start {
                self.chars.next();
            } else {
                break;
            } 
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = self.next_token()?;
            match token {
                Token::Eof => break,
                _ => tokens.push(token),
            }
        }
        Ok(tokens)
    }
    fn read_string(&mut self) -> Result<Token, LexerError> {
        let mut str = String::new();
        for ch in &mut self.chars {
            match ch {
                '"' => break,
                '\n' => return Err(LexerError::UnClosedString),
                _ => {
                    str.push(ch);
                }
            }
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
                },

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
        

        if has_dot {
            Ok(Token::Float(
                buf.parse::<f64>().map_err(LexerError::FloatUnKnow)?,
            ))
        } else {
            if buf.starts_with('0') {
                return Err(LexerError::CannotStartZeroNumber);
            }
            Ok(Token::Number(
                buf.parse::<i64>().map_err(LexerError::NumberUnKnow)?,
            ))
        }
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let char = self.chars.peek();
        if let Some('\n') = char {
            self.is_line_start = true;
            self.chars.next();
            return Ok(Token::Newline);
        }

        self.skip_whitespace();
        if self.current_indent > 0 && self.current_indent % 4 ==0 && self.is_line_start {
            self.pending_intend += 1;
            self.indent_stack.push(self.current_indent);
            self.current_indent = 0;
            self.is_line_start = false;
            return Ok(Token::Indent);
        }
     //   println!("Pending {} is empty? : {} {} {:?}",self.pending_intend, self.indent_stack.is_empty(), self.is_line_start, self.indent_stack.last());
        if self.pending_dedents>0 && !self.indent_stack.is_empty() && self.is_line_start && self.current_indent < *self.indent_stack.last().unwrap() {
            self.indent_stack.pop();
            self.pending_dedents -= 1;
            return Ok(Token::Dedent)
        }
        
     let char = self.chars.peek();
     let token = match char {
            Some('(') => {
                self.chars.next();
                Ok(Token::LParen)
            },
            Some(')') => {
                self.chars.next();
                Ok(Token::RParen)
            },
            Some(':') => Ok(Token::Colon),
            Some(',') => Ok(Token::Comma),
            Some('{') => Ok(Token::LBrace),
            Some('.') => Ok(Token::Dot),
            Some('}') => Ok(Token::RBrace),
            Some('_') => Ok(Token::Underscore),
            Some('[') => Ok(Token::ListStart),
            Some(']') => Ok(Token::ListEnd),
            Some('=') => {
                self.chars.next();
                Ok(Token::Op('='))
            },
            Some('/') => Ok(Token::Op('/')),
            Some('+') => Ok(Token::Op('+')),
            Some('-') => Ok(Token::Op('-')),
            Some('*') => Ok(Token::Op('*')),
            Some('0'..='9') => self.read_number(),
            Some('\'') | Some('"') => self.read_string(),
            Some(_) => self.read_word(),
            None => Ok(Token::Eof),
        };
        self.is_line_start =false;
        token
    }
}
