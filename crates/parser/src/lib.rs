pub mod assign;
pub mod ast;
pub mod binary_op;
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
pub mod shared_ast;
pub mod template;
pub mod types;
pub mod r#while_loop;

#[cfg(test)]
mod tests;

use crate::{ast::Statement, errors::ParserError, expressions::parse_expression_block};
#[derive(Default)]
pub struct ParsedProgram {
    pub modules: Vec<String>,
    pub ast: Vec<Statement>,
}
pub fn parser(sdk: String) -> Result<ParsedProgram, ParserError> {
    let mut lexer = tokenizer::Lexer::new(&sdk);

    let mut tokens = lexer.tokenize()?;
    let parsed = parse_expression_block(&mut tokens)?;
    Ok(parsed)
}
