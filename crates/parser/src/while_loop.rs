use crate::{
    ast::Statement, binary_op::parse_expression, condition::parse_block, errors::ParserError,
    helpers::expect_token,
};
use tokenizer::{iterator::Tokens, tokens::Token};

pub fn parse_while_loop(tokens: &mut Tokens) -> Result<Statement, ParserError> {
    tokens.next();
    let condition = parse_expression(tokens)?;
    expect_token(tokens, Token::Newline)?;
    let body = parse_block(tokens)?;
    Ok(Statement::While {
        condition: Box::new(condition),
        body,
    })
}
