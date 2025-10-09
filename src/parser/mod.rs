pub mod builtin;
pub mod decl;
pub mod enums;
mod expression;
pub mod function_def;
pub mod helper;
pub mod identifier;
pub mod if_expr;
pub mod list;
pub mod loops;
pub mod r#match;
pub mod method;
pub mod op_expr;
pub mod struct_init;
pub mod structs;
pub mod template;
pub mod types;
pub mod union;
use color_eyre::eyre::Result;
use peekmore::PeekMore;

use crate::{
    lexer::Token,
    parser::{ast::Program, expression::parse_expression_block},
};

pub mod ast;
#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a mut Vec<Token>,
}
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a mut Vec<Token>) -> Self {
        Self { tokens }
    }
    pub fn parse(&mut self) -> Result<Program> {
        let tokens = &mut self.tokens.iter().peekmore();
        let mut program = Program {
            expressions: Vec::new(),
            function_defs: Vec::new(),
        };
        let ast = parse_expression_block(tokens)?;

        program.expressions = ast;
        Ok(program)
    }
}
