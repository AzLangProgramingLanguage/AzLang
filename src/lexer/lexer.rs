use crate::parser::ast::Type;
use crate::translations::syntax::Syntax;
use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;

pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub syntax: &'a Syntax,
    token_buffer: Vec<Token>,
    indent_stack: Vec<usize>, // İndentasiya səviyyələrini izləmək üçün
    current_indent: usize,    // Cari sətirin indentasiya səviyyəsi
    pending_dedents: usize,   // Gözləyən dedent sayı
    at_line_start: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, syntax: &'a Syntax) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            syntax,
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
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }
        if !tokens.contains(&Token::EOF) {
            tokens.push(Token::EOF);
        }
        tokens
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if ch == ' ' && self.at_line_start {
                self.current_indent += 1;
                self.chars.next();
            } else if ch.is_whitespace() && ch != '\n' {
                // Sətir əvvəlində olmayan boşluqları keç
                self.chars.next();
            } else {
                break;
            }
        }
    }
    fn handle_dedent(&mut self) -> Option<Token> {
        let current_level = *self.indent_stack.last().unwrap();
        if self.current_indent < current_level {
            self.indent_stack.pop();

            // Yalnız 1 Dedent qaytar, qalanını pending_dedents-ə yaz
            self.pending_dedents = self
                .indent_stack
                .iter()
                .filter(|&&level| level > self.current_indent)
                .count();

            Some(Token::Dedent)
        } else {
            None
        }
    }
    pub fn next_token(&mut self) -> Option<Token> {
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

        let ch = *self.chars.peek()?;

        // Sətir əvvəlində indentasiya emalı
        if self.at_line_start && ch != '\n' {
            self.at_line_start = false;

            if self.current_indent > *self.indent_stack.last().unwrap() {
                self.indent_stack.push(self.current_indent);
                self.current_indent = 0;
                return Some(Token::Indent);
            } else if self.current_indent < *self.indent_stack.last().unwrap() {
                return self.handle_dedent();
            }
            // Eyni səviyyədədirsə, heç nə etmə
        }

        match ch {
            '\n' => {
                self.chars.next();
                self.at_line_start = true;

                // Yeni sətrin indentini oxu
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
            // ... digər token emalları eyni qalır ...
            _ => {
                self.at_line_start = false;
                match ch {
                    '.' => {
                        self.chars.next();
                        Some(Token::Dot)
                    }
                    '(' => self.consume_char_and_return(Token::LParen),
                    ')' => self.consume_char_and_return(Token::RParen),
                    '{' => self.consume_char_and_return(Token::LBrace),
                    ';' => self.consume_char_and_return(Token::Semicolon),
                    ':' => self.consume_char_and_return(Token::Colon),
                    ',' => self.consume_char_and_return(Token::Comma),
                    '[' => self.consume_char_and_return(Token::ListStart),
                    ']' => self.consume_char_and_return(Token::ListEnd),
                    '`' => {
                        self.chars.next();
                        self.read_template_string()
                    }
                    '$' => self.read_template_string(),
                    '}' => {
                        self.chars.next();
                        Some(Token::RBrace)
                    }
                    '"' => self.read_string(),
                    '0'..='9' => self.read_number(),
                    _ if ch.is_alphabetic() || ch == '_' => self.read_word(),
                    _ => self.read_operator(),
                }
            }
        }
    }

    fn read_word(&mut self) -> Option<Token> {
        let mut word = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.is_alphabetic() || ch == '_' {
                word.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }

        // Keyword yoxlamaları

        if word == self.syntax.return_name {
            Some(Token::Return)
        } else if word == self.syntax.this_str {
            Some(Token::This)
        } else if word == self.syntax.mutable_decl {
            Some(Token::MutableDecl)
        } else if word == self.syntax.method_str {
            Some(Token::Method)
        } else if word == self.syntax.object_str {
            Some(Token::Object)
        } else if word == self.syntax.end_str {
            Some(Token::End)
        } else if word == self.syntax.constant_decl {
            Some(Token::ConstantDecl)
        } else if word == self.syntax.break_str {
            Some(Token::Break)
        } else if word == self.syntax.continue_str {
            Some(Token::Continue)
        } else if word == self.syntax.true_str {
            Some(Token::True)
        } else if word == self.syntax.false_str {
            Some(Token::False)
        } else if word == self.syntax.in_str {
            Some(Token::In)
        } else if word == self.syntax.function_def {
            Some(Token::FunctionDef)
        } else if word == self.syntax.else_if {
            Some(Token::ElseIf)
        } else if word == self.syntax.conditional {
            Some(Token::Conditional)
        } else if word == self.syntax._else {
            Some(Token::Else)
        } else if word == self.syntax._loop {
            Some(Token::Loop)
        } else if word == self.syntax.bool {
            return Some(Token::TypeName(Type::Bool));
        } else if word == self.syntax.listtype {
            return Some(Token::SiyahiKeyword);
        } else if word == self.syntax.biginteger {
            return Some(Token::TypeName(Type::BigInteger));
        } else if word == self.syntax.lowinteger {
            return Some(Token::TypeName(Type::LowInteger));
        } else if word == self.syntax.string {
            return Some(Token::TypeName(Type::Metn));
        } else if word == self.syntax.integer {
            return Some(Token::TypeName(Type::Integer));
        } else if self.syntax.is_type_str(&word) {
            return Some(Token::TypeName(Type::Istifadeci(word)));
        } else if word == self.syntax.string {
            return Some(Token::String);
        } else {
            Some(Token::Identifier(word))
        }
    }

    fn consume_char_and_return(&mut self, token: Token) -> Option<Token> {
        self.chars.next();
        Some(token)
    }

    pub fn push_back_token(&mut self, token: Token) {
        self.token_buffer.push(token);
    }

    /*   fn read_dollar(&mut self) -> Option<Token> {
        self.chars.next();
        self.read_template_string()
    } */

    fn read_string(&mut self) -> Option<Token> {
        self.chars.next(); // Skip opening quote
        let mut string = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '"' {
                self.chars.next(); // Skip closing quote
                return Some(Token::StringLiteral(string));
            }
            string.push(ch);
            self.chars.next();
        }

        None // Unterminated string
    }
    fn read_template_string(&mut self) -> Option<Token> {
        let mut content = String::new();

        while let Some(&ch) = self.chars.peek() {
            match ch {
                '`' => {
                    self.chars.next(); // Bitirici backtick
                    if !content.is_empty() {
                        return Some(Token::TemplatePart(content));
                    } else {
                        return Some(Token::Backtick);
                    }
                }
                '$' => {
                    let mut lookahead = self.chars.clone();
                    lookahead.next(); // $
                    if let Some('{') = lookahead.next() {
                        // Önce içerideki content varsa onu döndür
                        if !content.is_empty() {
                            let part = Token::TemplatePart(content);
                            /*   content = String::new(); */
                            self.chars.next(); // $
                            self.chars.next(); // {
                            self.push_back_token(Token::InterpolationStart);
                            return Some(part);
                        }
                        self.chars.next(); // $
                        self.chars.next(); // {
                        return Some(Token::InterpolationStart);
                    } else {
                        // Normal $ karakteri
                        content.push('$');
                        self.chars.next();
                    }
                }
                '}' => {
                    self.chars.next(); // }
                    return Some(Token::RBrace);
                }
                _ => {
                    content.push(ch);
                    self.chars.next();
                }
            }
        }

        // Eğer döngü biterse ve hala content varsa
        if !content.is_empty() {
            Some(Token::TemplatePart(content))
        } else {
            None
        }
    }

    fn read_number(&mut self) -> Option<Token> {
        let mut num_str = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch.is_digit(10) {
                num_str.push(ch);
                self.chars.next();
            } else {
                break;
            }
        }

        num_str.parse().ok().map(Token::Number)
    }

    fn read_operator(&mut self) -> Option<Token> {
        let mut op = String::new();
        op.push(self.chars.next()?);

        // Çok karakterli operatörler için (örneğin ==, +=)
        if let Some(&next_ch) = self.chars.peek() {
            match (op.chars().next().unwrap(), next_ch) {
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
                _ => {}
            }
        }

        Some(Token::Operator(op))
    }
}
