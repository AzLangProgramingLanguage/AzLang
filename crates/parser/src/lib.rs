pub mod assign;
pub mod ast;
pub mod binary_op;
pub mod condition;
pub mod decl;
pub mod errors;
mod expressions;
pub mod function;
pub mod helpers;
pub mod identifier;
pub mod list;
pub mod literal_parse;
pub mod r#loop;
pub mod shared_ast;
pub mod template;
pub mod types;
pub mod r#while_loop;

#[cfg(test)]
mod tests;

use crate::{ast::Statement, errors::ParserError, expressions::parse_expression_block};

pub fn parser(sdk: String) -> Result<Vec<Statement>, ParserError> {
    let mut lexer = tokenizer::Lexer::new(&sdk);
    let mut tokens = lexer.tokenize()?;
    let ast = parse_expression_block(&mut tokens)?;
    Ok(ast)
}
