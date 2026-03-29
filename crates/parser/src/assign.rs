use std::borrow::Cow;

use crate::{
    ast::{Expr, Statement, Symbol},
    binary_op::parse_expression,
    errors::ParserError,
    expressions::parse_single_expr,
    helpers::expect_token,
    shared_ast::Type,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_assign(tokens: &mut Tokens, s: String) -> Result<Statement, ParserError> {
    expect_token(tokens, Token::Assign)?;
    let value = parse_expression(tokens)?;
    Ok(Statement::Assignment {
        name: s,
        value: Box::new(value),
        symbol: None,
    })
}
