use std::{iter::Peekable, str::Chars};

use crate::{
    errors::LexerError,
    iterator::{SourceSpan, Tokens},
    tokens::Token,
    words::tokenize_word,
};
pub mod errors;
pub mod iterator;
pub mod token_display;
pub mod tokens;
pub mod words;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    is_line_start: bool,
    indent_level: usize,
    space: usize,
    line: u32,
    start: u32,
    end: u32,
    token_buffer: Vec<(Token,SourceSpan)>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            is_line_start: false,
            indent_level: 0,
            space: 0,
            line: 1,
            start: 1,
            end: 1,
            token_buffer: Vec::new(),
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
            while let Some((token,span)) = self.token_buffer.pop() {
                tokens.push(token,span);
            }
        }

        Ok(tokens)
    }

    fn read_template_string(&mut self) -> Result<Token, LexerError> {
        self.chars.next();
        let mut str = String::new();

        loop {
            let ch = self.chars.peek();
            println!("Char {:?}", ch);
            match ch {
                Some('`') => {
                    self.token_buffer.push((Token::Backtick, SourceSpan {
                        start: self.start,
                        end: self.end,
                        line: self.line,
                    }));
                    self.chars.next();
                    break;
                }
                Some('$') => {
                    self.chars.next();
                    if let Some('{') = self.chars.peek() {
                        self.chars.next();
                        self.end += 2;
                        self.token_buffer.push((Token::StringLiteral(std::mem::take(&mut str)), SourceSpan {
                            start: self.start,
                            end: self.end,
                            line: self.line,
                        }));
                        self.token_buffer.push((Token::InterpolationStart, SourceSpan {
                            start: self.start,
                            end: self.end,
                            line: self.line,
                        }));
                        str.clear();

                        while let Some(token) = self.chars.peek() {
                            match *token {
                                '}' => {
                                    self.chars.next();
                                    self.token_buffer.push((Token::InterpolationEnd, SourceSpan {
                                        start: self.start,
                                        end: self.end,
                                        line: self.line,
                                    }));
                                    break;
                                }
                                _ => {
                                    let token = self.next_token()?;
                                   
                                    self.end += 1;
                                    self.token_buffer.push((token, SourceSpan {
                                        start: self.start,
                                        end: self.end,
                                        line: self.line,
                                    }));
                                }
                            }                        
                        }
                    }
                    else {
                        self.chars.next();
                        self.end += 1;
                        str.push('$');
                    }
                }
             
                Some(ch) => {
                    self.end += 1;
                    str.push(*ch);
                    self.chars.next();
                }
                None => return Err(LexerError::UnClosedString(
                    SourceSpan {
                        start: self.start,
                        end: self.end, 
                        line: self.line,
                    },
                    str,
                )),
            }
        }
        
       Ok(Token::Backtick)
    }

    fn read_string(&mut self) -> Result<Token, LexerError> {
        self.chars.next();
     
        let mut str = String::new();
        let mut is_closed = false;

        for ch in &mut self.chars {
            match ch {
                '"' => {
                    is_closed = true;
                    break;
                },
                '\n' => {
                    return Err(LexerError::UnClosedString(
                        SourceSpan {
                            start: self.start,
                            end: self.end, 
                            line: self.line,
                        },
                        str,
                    ));
                }
                other => {
                    str.push(other);
                }
            }

            self.end += 1;
        }
        if !is_closed {
            return Err(LexerError::UnClosedString(
                SourceSpan {
                    start: self.start,
                    end: self.end, 
                    line: self.line,
                },
            str,
        ));
        }

        Ok(Token::StringLiteral(str))
    }
    fn read_word(&mut self) -> Result<Token, LexerError> {

        let mut str = String::new();
        while let Some(ch) = self.chars.peek() {
            if ch.is_alphanumeric() {
                self.end += 1;
                str.push(*ch);
                self.chars.next();
            } else {
                break;
            }
        }
        self.end += 1;
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
                return Err(LexerError::CannotStartZeroNumber(
                    SourceSpan {
                        start: self.start,
                        end: self.end,
                        line: self.line,
                    },
                    buf,
                ));
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
            Some('%') => self.consume(Token::Op('%')),
            Some('^') => self.consume(Token::Op('^')),
            Some('>') => self.consume(Token::Op('>')),
            Some('<') => self.consume(Token::Op('<')),
            Some('`') => self.read_template_string(),
            Some('0'..='9') => self.read_number(),
            Some('\'') | Some('"') => self.read_string(),
            Some(_) => self.read_word(),
            None => Ok(Token::Eof),
        };
        self.is_line_start = false;
        token
    }
}