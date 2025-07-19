use peekmore::PeekMoreIterator;

use color_eyre::eyre::{Result, eyre};

use crate::lexer::Token;

pub fn skip_newlines<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<()>
where
    I: Iterator<Item = &'a Token>,
{
    while matches!(tokens.peek(), Some(Token::Newline)) {
        tokens.next();
    }
    Ok(())
}

pub fn expect_token<'a, I>(tokens: &mut PeekMoreIterator<I>, expected: Token) -> Result<()>
where
    I: Iterator<Item = &'a Token>,
{
    match tokens.next() {
        Some(t) if *t == expected => Ok(()),
        other => Err(eyre!("Gözlənilirdi {:?}, tapıldı {:?}", expected, other)),
    }
}
