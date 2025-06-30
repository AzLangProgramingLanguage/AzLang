pub mod ast;

use std::collections::{HashMap, HashSet};

use crate::{Symbol, lexer::Token, parser::ast::Type};
pub use ast::{Expr, Program};

// Digər modulları elan et
pub mod builtin;
pub mod call;
pub mod r#enum;
pub mod expressions;
pub mod function;
pub mod if_expr;
pub mod list;
pub mod loop_expr;
pub mod r#match;
pub mod method;
pub mod object;
pub mod returnn;
mod statements;
pub mod template;
pub mod types;

pub struct Parser {
    tokens: Vec<Token>,
    pub position: usize, // Testing üçün pub edək, sonra private ola bilər
    pub declared_variables: HashMap<String, Symbol>,
    pub used_variables: HashSet<String>,
    pub current_function: Option<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            declared_variables: HashMap::new(), //
            used_variables: HashSet::new(),
            current_function: None,
        }
    }
    pub fn next_if(&mut self, token: Token) -> Option<&Token> {
        if self.peek() == Some(&token) {
            self.next()
        } else {
            None
        }
    }
    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.position + n)
    }
    pub fn next_identifier(&mut self) -> Result<String, String> {
        match self.next() {
            Some(Token::Identifier(name)) => Ok(name.clone()),
            other => Err(format!(
                "Tanıtıcı (identifier) gözlənilirdi, tapıldı: {:?}",
                other
            )),
        }
    }
    pub fn expect(&mut self, expected: &Token) -> Result<(), String> {
        match self.peek() {
            Some(t) if t == expected => {
                self.next(); // Tokeni alır
                Ok(())
            }
            Some(t) => Err(format!(
                "'{:?}' gözlənilirdi, amma '{:?}' tapıldı",
                expected, t
            )),
            None => Err(format!(
                "'{:?}' gözlənilirdi, amma fayl sonuna çatıldı",
                expected
            )),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        tok
    }

    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        self.position += 1;
        tok
    }

    /// Yalnız parse_program çağırılır
    pub fn parse(&mut self) -> Result<Program, String> {
        let expressions = self.parse_program()?;
        Ok(Program {
            expressions,
            return_type: Some(Type::Void),
        })
    }

    fn parse_statement(&mut self) -> Result<Option<Expr>, String> {
        statements::parse_statement(self)
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        expressions::parse_expression(self, false)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        types::parse_type(self)
    }

    pub fn parse_program(&mut self) -> Result<Vec<Expr>, String> {
        // Boş sətirləri keç
        while let Some(Token::Newline) = self.peek() {
            self.next();
        }

        let mut statements = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Indent | Token::Dedent | Token::Newline => {
                    self.next(); // bu tokenləri buradan skip elə
                    continue;
                }
                Token::EOF => {
                    break;
                }

                _ => {
                    if let Some(stmt) = self.parse_statement()? {
                        statements.push(stmt);

                        // Semicolon varsa keç
                        if let Some(Token::Semicolon) = self.peek() {
                            self.next();
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        Ok(statements)
    }
}
