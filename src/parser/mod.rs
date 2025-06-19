pub mod ast;

use std::collections::HashSet;

use crate::{context::TranspileContext, lexer::Token, parser::ast::Type};
pub use ast::{Expr, Program};

// Digər modulları elan et
pub mod builtin;
pub mod call;
pub mod expressions;
pub mod function;
pub mod if_expr;
pub mod list;
pub mod loop_expr;
pub mod object;
pub mod returnn;

mod statements;
pub mod types;

pub struct Parser {
    tokens: Vec<Token>,
    pub position: usize, // Testing üçün pub edək, sonra private ola bilər
    pub declared_variables: HashSet<String>,
    pub used_variables: HashSet<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
            declared_variables: HashSet::new(),
            used_variables: HashSet::new(),
        }
    }
    pub fn next_if(&mut self, token: Token) -> Option<&Token> {
        if self.peek() == Some(&token) {
            self.next()
        } else {
            None
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
        /*         println!("peek() → position = {}, token = {:?}", self.position, tok);
         */
        tok
    }

    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        /*         println!("next() → position = {}, token = {:?}", self.position, tok);
         */
        self.position += 1;
        tok
    }

    /// Yalnız parse_program çağırılır
    pub fn parse(&mut self, ctx: &mut TranspileContext) -> Result<Program, String> {
        let expressions = self.parse_program(ctx)?;
        Ok(Program { expressions })
    }

    fn parse_statement(&mut self, ctx: &mut TranspileContext) -> Result<Option<Expr>, String> {
        statements::parse_statement(self, ctx)
    }

    fn parse_expression(&mut self, ctx: &mut TranspileContext) -> Result<Expr, String> {
        expressions::parse_expression(self, false, ctx)
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        types::parse_type(self)
    }

    pub fn parse_program(&mut self, ctx: &mut TranspileContext) -> Result<Vec<Expr>, String> {
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
                    if let Some(stmt) = self.parse_statement(ctx)? {
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
