pub mod ast;
pub mod binary_op;
pub mod builtin;
pub mod condition;
mod expressions;
pub mod function;
pub mod helpers;
pub mod list;
pub mod r#loop;
pub mod method;
pub mod structs;
pub mod template;
pub mod types;
use ast::Program;
use errors::ParserError;
use peekmore::PeekMore;
use tokenizer::tokens::Token;

use crate::parser::expressions::parse_expression_block;
pub struct Parser<'a> {
    tokens: &'a mut Vec<Token>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token>) -> Self {
        Self { tokens }
    }
    pub fn parse(&self) -> Result<Program, ParserError> {
        let tokens = &mut self.tokens.iter().peekmore();
        let ast = parse_expression_block(tokens)?;
        Ok(Program {
            function_defs: Vec::new(),
            expressions: ast,
        })
    }
}
