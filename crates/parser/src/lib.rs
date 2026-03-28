pub mod ast;
pub mod binary_op;
pub mod builtin;
pub mod condition;
pub mod decl;
pub mod function;
pub mod helpers;
pub mod identifier;
pub mod list;
pub mod literal_parse;
pub mod r#loop;
pub mod template;
pub mod types;

/*
pub mod builtin;
pub mod r#enum; */
pub mod errors;
mod expressions;
/*


pub mod r#match;
pub mod method; */
pub mod shared_ast;
/* pub mod struct_init;
pub mod structs; */
/*
 */
mod tests;
/*
 *//* pub mod union; */
use crate::{errors::ParserError, expressions::parse_expression_block};
use ast::Program;

pub fn parser(sdk: String) -> Result<Program, ParserError> {
    let mut lexer = tokenizer::Lexer::new(&sdk);
    let mut tokens = lexer.tokenize()?;
    let ast = parse_expression_block(&mut tokens)?;
    Ok(ast)
}
