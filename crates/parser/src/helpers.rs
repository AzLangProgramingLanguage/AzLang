use crate::{errors::ParserError};
use tokenizer::{iterator::{SpannedToken, Tokens}, tokens::Token};

pub fn skip_newlines<'a>(tokens: &mut Tokens) -> Result<(), ParserError>
{
    while matches!(tokens.peek(), Some(SpannedToken{ token: Token::Newline, .. })) {
        tokens.next();
    }
    Ok(())
}

pub fn expect_token<'a>(
    tokens: &mut Tokens,
    expected: Token,
) -> Result<(), ParserError>
{
    match tokens.next() {
        Some(SpannedToken{ token: t, .. }) if t == expected => Ok(()),
        None => Err(ParserError::UnexpectedEOF),
        Some(other) => Err(ParserError::ExpectedToken(expected, other.token)),
    }
}
