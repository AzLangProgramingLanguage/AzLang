pub mod builtin;
pub mod decl;
pub mod enums;
mod expression;
pub mod function_def;
pub mod helper;
pub mod if_expr;
pub mod list;
pub mod loops;
pub mod r#match;
pub mod method;
pub mod object;
pub mod op_expr;
pub mod parse_identifier;
pub mod parse_union_type;
pub mod structs;
pub mod template;
pub mod types;
use color_eyre::eyre::Result;
use peekmore::PeekMore;

use crate::{
    lexer::Token,
    parser::{ast::Program, expression::parse_expression_block},
};

pub mod ast;
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    is_used_allocator: bool,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            is_used_allocator: false,
        }
    }
    pub fn parse(&mut self) -> Result<Program> {
        let tokens = &mut self.tokens.iter().peekmore();

        let ast = parse_expression_block(tokens)?;

        let program = Program { expressions: ast };
        Ok(program)
    }
}
