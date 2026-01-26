pub mod ast;
pub mod literal_parse;
pub mod builtin;
pub mod template;
pub mod binary_op;
pub mod types;
pub mod decl;
pub mod list;

pub mod helpers;
/* 
pub mod builtin;
pub mod condition;
pub mod r#enum; */
pub mod errors;
mod expressions;
/* pub mod function;
pub mod identifier;

pub mod r#loop;
pub mod r#match;
pub mod method; */
pub mod shared_ast;
/* pub mod struct_init;
pub mod structs; */
/* 
 */mod tests;
/* 
 *//* pub mod union; */
use crate::expressions::parse_expression_block;
use ast::Program;
use tokenizer::iterator::Tokens;
#[derive(Debug)]
pub struct Parser {
    tokens: Tokens,
}
impl Parser {
    pub fn new(tokens: Tokens) -> Self {
        Self {
            tokens,
        }
    }
    pub fn parse(&mut self) -> Result<Program<'_>, errors::ParserError> {
        let ast = parse_expression_block(&mut self.tokens)?;
        Ok(Program { expressions: ast })
    }
}
