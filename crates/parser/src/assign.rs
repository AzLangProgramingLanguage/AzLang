use crate::{
    ast::{Atom, Expr, Statement},
    binary_op::parse_expression,
    errors::ParserError,
    helpers::expect_token,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_assign(tokens: &mut Tokens, s: String) -> Result<Statement, ParserError> {
    tokens.next();
    expect_token(tokens, Token::Assign)?;
    let value = parse_expression(tokens)?;
    Ok(Statement::Assignment {
        name: Atom::from(s),
        value: Box::new(value),
    })
}
