pub mod ast;
pub mod binary_op;
pub mod builtin;
pub mod condition;
pub mod decl;
pub mod r#enum;
pub mod errors;
mod expressions;
pub mod function;
pub mod helpers;
pub mod identifier;
pub mod list;
pub mod literal_parse;
pub mod r#loop;
pub mod r#match;
pub mod method;
pub mod shared_ast;
pub mod struct_init;
pub mod structs;
pub mod template;
mod tests;
pub mod types;
pub mod union;
use ast::Program;
use peekmore::PeekMore;
use tokenizer::tokens::Token;

use crate::expressions::parse_expression_block;
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}
impl Parser {
    pub fn new(string: String) -> Self {
        let tokens = tokenizer::Lexer::new(&string).tokenize();
        Self { tokens }
    }
    pub fn parse(&mut self) -> Result<Program<'_>, errors::ParserError> {
        let ast = parse_expression_block(&mut self.tokens.iter().peekmore())?;
        Ok(Program {
            function_defs: Vec::new(),
            expressions: ast,
        })
    }
}
