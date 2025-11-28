use crate::{errors::ParserError, shared_ast::Type};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, expressions::parse_single_expr, list::parse_list};

pub fn skip_newlines<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<(), ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    while matches!(tokens.peek(), Some(Token::Newline)) {
        tokens.next();
    }
    Ok(())
}

pub fn expect_token<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    expected: Token,
) -> Result<(), ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(t) if *t == expected => Ok(()),
        None => Err(ParserError::UnexpectedEOF),
        Some(other) => Err(ParserError::ExpectedToken(expected, other.clone())),
    }
}
