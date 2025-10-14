use peekmore::{PeekMore, PeekMoreIterator};

use crate::lexer::words::tokenize_word;
use std::mem;
use std::str::Chars;

use super::token::Token;

pub struct Lexer<'a> {
    pub chars: PeekMoreIterator<Chars<'a>>,
    token_buffer: Vec<Token>,
    indent_stack: Vec<usize>, // İndentasiya səviyyələrini izləmək üçün
    current_indent: usize,    // Cari sətirin indentasiya səviyyəsi
    pending_dedents: usize,   // Gözləyən dedent sayı
    at_line_start: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekmore(),
            token_buffer: Vec::new(),
            indent_stack: vec![0],
            current_indent: 0,
            pending_dedents: 0,
            at_line_start: true,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            if token == Token::Eof {
                break;
            }
            tokens.push(token);
        }
        if !tokens.contains(&Token::Eof) {
            tokens.push(Token::Eof);
        }
        tokens
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch == ' ' && self.at_line_start {
                self.current_indent += 1;
                self.chars.next();
            } else if ch.is_whitespace() && ch != '\n' {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn handle_dedent(&mut self) -> Option<Token> {
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Some(Token::Dedent);
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
            return Some(Token::Dedent);
        }

        None
    }
    fn skip_comment_line(&mut self) {
        let mut commentline = String::new();
        while let Some(&ch) = self.chars.peek() {
            if ch == '\n' {
                break; // sətrin sonuna qədər ignore elə
            }
            commentline.push(ch);
            self.chars.next();
        }
        self.token_buffer.push(Token::Comment(commentline));
    }
    fn skip_comment_block(&mut self) {
        while let Some(ch) = self.chars.next() {
            if ch == '*' {
                if let Some(&'/') = self.chars.peek() {
                    self.chars.next();
                    return;
                }
            }
        }
    }
    fn next_token(&mut self) -> Option<Token> {
        // Əvvəlcə gözləyən dedentləri yoxla
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Some(Token::Dedent);
        }

        // Bufferdə token varsa onu qaytar
        if let Some(token) = self.token_buffer.pop() {
            return Some(token);
        }

        self.skip_whitespace();

        let ch = *self.chars.peek()?; // Eof yoxlaması

        // Sətir əvvəlində indent emalı
        if self.at_line_start && ch != '\n' {
            self.at_line_start = false;

            if self.current_indent > *self.indent_stack.last().unwrap() {
                self.indent_stack.push(self.current_indent);
                self.current_indent = 0;
                return Some(Token::Indent);
            } else if self.current_indent < *self.indent_stack.last().unwrap() {
                return self.handle_dedent();
            }
        }

        match ch {
            '\n' => {
                self.chars.next();
                self.at_line_start = true;

                let mut count = 0;
                while let Some(&' ') = self.chars.peek() {
                    self.chars.next();
                    count += 1;
                }

                self.current_indent = count;
                let last_indent = *self.indent_stack.last().unwrap();

                if self.current_indent > last_indent {
                    self.indent_stack.push(self.current_indent);
                    self.token_buffer.push(Token::Indent);
                }

                Some(Token::Newline)
            }
            '.' => self.consume_char_and_return(Token::Dot),
            '(' => self.consume_char_and_return(Token::LParen),
            ')' => self.consume_char_and_return(Token::RParen),
            '{' => self.consume_char_and_return(Token::LBrace),
            '}' => self.consume_char_and_return(Token::RBrace),
            ';' => self.consume_char_and_return(Token::Semicolon),
            '_' => self.consume_char_and_return(Token::Underscore),
            ':' => self.consume_char_and_return(Token::Colon),
            ',' => self.consume_char_and_return(Token::Comma),
            '[' => self.consume_char_and_return(Token::ListStart),
            ']' => self.consume_char_and_return(Token::ListEnd),
            '/' => {
                self.chars.next();
                if let Some(&'/') = self.chars.peek() {
                    self.chars.next(); // ikinci `/`-i də keç
                    self.skip_comment_line();
                    self.next_token() // koment bitəndən sonra növbəti tokeni qaytar
                } else if let Some(&'*') = self.chars.peek() {
                    self.chars.next(); // ikinci `*`-i də keç
                    self.skip_comment_block();
                    self.next_token() // koment bitəndən sonra növbəti tokeni qaytar
                } else {
                    Some(Token::Operator("/".to_string()))
                }
            }
            '`' => {
                self.chars.next(); // skip backtick
                let tokens = self.read_template_string(); // Vec<Token>
                self.token_buffer.extend(tokens.into_iter().rev());
                self.next_token()
            }
            '"' | '\'' => self.read_string(),
            '0'..='9' => self.read_number(),
            _ if ch.is_alphabetic() || ch == '_' => self.read_word(),
            _ => self.read_operator(),
        }
    }

    fn read_word(&mut self) -> Option<Token> {
        let mut word = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                word.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }
        Some(tokenize_word(&word))
    }

    fn consume_char_and_return(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    fn read_string(&mut self) -> Option<Token> {
        let quote = self.chars.next()?;
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == quote {
                self.chars.next();
                return Some(Token::StringLiteral(string));
            }
            if ch == '\\' {
                self.chars.next();
                if let Some(&escaped_ch) = self.chars.peek() {
                    string.push(escaped_ch);
                    self.chars.next();
                }
                continue;
            }

            string.push(ch);
            self.chars.next();
        }

        None
    }

    fn read_template_string(&mut self) -> Vec<Token> {
        let mut tokens = vec![Token::Backtick];
        let mut current = String::new();

        while let Some(ch) = self.chars.next() {
            match ch {
                '`' => {
                    if !current.is_empty() {
                        let takes = mem::take(&mut current);
                        tokens.push(Token::StringLiteral(takes));
                    }
                    tokens.push(Token::Backtick);
                    break;
                }
                '$' => {
                    if let Some(&'{') = self.chars.peek() {
                        self.chars.next(); // skip {
                        if !current.is_empty() {
                            let takes = mem::take(&mut current);
                            tokens.push(Token::StringLiteral(takes));
                        }
                        tokens.push(Token::InterpolationStart);

                        // 🔥 yeni expression oxuma
                        let expr_tokens = self.read_interpolated_expr_tokens();
                        tokens.extend(expr_tokens);

                        tokens.push(Token::InterpolationEnd);
                    } else {
                        current.push(ch);
                    }
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(Token::StringLiteral(current));
        }

        tokens
    }

    fn read_number(&mut self) -> Option<Token> {
        let mut num_str = String::new();
        let mut has_dot = false;

        while let Some(&ch) = self.chars.peek() {
            match ch {
                '0'..='9' => {
                    num_str.push(ch);
                    self.chars.next();
                }
                '.' if !has_dot => {
                    if let Some(&next_ch) = self.chars.peek_nth(1) {
                        if next_ch.is_ascii_digit() {
                            has_dot = true;
                            num_str.push(ch);
                            self.chars.next();
                        } else {
                            break; // nöqtədən sonra rəqəm yoxdursa, ayrı token
                        }
                    } else {
                        break; // son char `.` idisə, float etmə
                    }
                }
                _ => break,
            }
        }

        if has_dot {
            num_str.parse::<f64>().ok().map(Token::Float)
        } else {
            num_str.parse::<i64>().ok().map(Token::Number)
        }
    }

    fn read_operator(&mut self) -> Option<Token> {
        let mut op = String::new();
        let first = self.chars.next()?;
        op.push(first);

        if let Some(&next_ch) = self.chars.peek() {
            match (first, next_ch) {
                ('=', '=')
                | ('!', '=')
                | ('<', '=')
                | ('>', '=')
                | ('+', '=')
                | ('-', '=')
                | ('*', '=')
                | ('/', '=')
                | ('&', '&')
                | ('|', '|') => {
                    op.push(next_ch);
                    self.chars.next();
                }
                ('-', '>') => {
                    op.push(next_ch);
                    self.chars.next();
                    return Some(Token::Arrow); // <<< Əsas düzəliş burada
                }
                _ => {}
            }
        }
        Some(Token::Operator(op))
    }

    fn read_interpolated_expr_tokens(&mut self) -> Vec<Token> {
        let mut expr = String::new();
        let mut brace_level = 1;

        while let Some(ch) = self.chars.next() {
            match ch {
                '{' => {
                    brace_level += 1;
                    expr.push(ch);
                }
                '}' => {
                    brace_level -= 1;
                    if brace_level == 0 {
                        break;
                    }
                    expr.push(ch);
                }
                _ => expr.push(ch),
            }
        }

        let mut tokens = Vec::new();
        let mut inner_lexer = Lexer::new(&expr);
        tokens.extend(
            inner_lexer
                .tokenize()
                .into_iter()
                .filter(|t| *t != Token::Eof),
        );

        tokens
    }
}
