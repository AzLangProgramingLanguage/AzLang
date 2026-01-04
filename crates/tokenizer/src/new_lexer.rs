use std::{iter::Peekable, str::Chars};
use crate::tokenize_word;
use crate::tokens::Token;
use crate::errors::LexerError;

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
    current_indent: usize
}
impl<'a> NewLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            is_line_start: false,
            indent_stack: vec![],
            pending_dedents:0,
            pending_intend:0,
            current_indent: 0
        }
    }
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch == ' ' && self.is_line_start {
                self.current_indent += 1;
                self.chars.next();
            } else if ch.is_whitespace() && ch != '\n' {
                self.chars.next();
            } else {
                break;
            }
        }
    }
    fn handle_dedent(&mut self) -> Result<Token,LexerError> {
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Ok(Token::Dedent);
        }

        let current_level = *self.indent_stack.last().unwrap();
        if self.current_indent < current_level {
            while let Some(&last) = self.indent_stack.last() {
                if last > self.current_indent {
                    self.indent_stack.pop();
                    self.pending_dedents += 1;
                } else {
                    break;
                }
            }

            self.pending_dedents -= 1;
            return Ok(Token::Dedent);
        }

        Err(LexerError::UnClosedString) /* BUG: Bug */
    }
    pub fn tokenize(&mut self)->Result<Vec<Token>,LexerError> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            let token = self.next_token()?;
            match token { 
              Token::Eof => break,
              _ =>  tokens.push(token)

            }
        } 
        Ok(tokens)
    }
    fn read_string(&mut self) -> Result<Token,LexerError> {
      let mut str = String::new();
      while let Some(ch) = self.chars.next() {
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
    fn read_word(&mut self,char:char) -> Result<Token,LexerError> {
       let mut str = char.to_string();
       while let Some(ch) = self.chars.peek() {
           if ch.is_alphanumeric() {
               str.push(*ch);
               self.chars.next();
           }else {
               break
           }
       }
       Ok(tokenize_word(str.as_str()))

    }
    
    fn read_number(&mut self, first: char) -> Result<Token, LexerError> {
       
        let mut buf = String::from(first);
        let mut has_dot = false;
    
        for ch in self.chars.by_ref() {
            match ch {
                '0'..='9' => buf.push(ch),
    
                'A'..='Z' | 'a'..='z' => {
                    return Err(LexerError::NumberAndAlpha);
                }
    
                '.' if !has_dot => {
                    has_dot = true;
                    buf.push('.');
                }
    
    
                _ => break,
            }
        }
    
        if has_dot {
           
            Ok(Token::Float(
                buf.parse::<f64>().map_err(LexerError::FloatUnKnow)?
            ))
        } else {
            if first =='0' {
                return Err(LexerError::CannotStartZeroNumber);
            }
            Ok(Token::Number(
                buf.parse::<i64>().map_err(LexerError::NumberUnKnow)?
            ))
        }
    }
    
    fn next_token(&mut self) -> Result<Token,LexerError> {
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Ok(Token::Dedent);
        }
        self.skip_whitespace();
        let ch = self.chars.peek();
        if self.is_line_start && *ch.unwrap() != '\n' {
            self.is_line_start = false;

            if self.current_indent > *self.indent_stack.last().expect("Error var bu setirde") {
                self.indent_stack.push(self.current_indent);
                self.current_indent = 0;
                return Ok(Token::Indent);
            } else if self.current_indent < *self.indent_stack.last().unwrap() {
                return self.handle_dedent();
            }
        }
        let token  = self.chars.next();
        match token {
            Some('\n') => {
                self.is_line_start = true;
                Ok(Token::Newline)
            },
            
            Some('(') => Ok(Token::LParen),
            Some(')') => Ok(Token::RParen),
            Some(':') => Ok(Token::Colon),
            Some(',') => Ok(Token::Comma),
            Some('{') => Ok(Token::LBrace),
            Some('.') => Ok(Token::Dot),
            Some('}') => Ok(Token::RBrace),
            Some('_') => Ok(Token::Underscore),
            Some('[') => Ok(Token::ListStart),
            Some(']') => Ok(Token::ListEnd),
            Some('0'..='9') => self.read_number(token.unwrap()),
            Some('\'') | Some('"') => self.read_string(),
            Some(other) => self.read_word(other),
            None => Ok(Token::Eof),
        }
    }
}
