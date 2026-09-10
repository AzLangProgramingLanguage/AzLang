use std::borrow::Cow;

use crate::{
    ast::{Expr, Statement},
    errors::ParserError,
    helpers::expect_token,
};
use string_cache::{Atom, EmptyStaticAtomSet};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};
pub fn parse_enum_decl(tokens: &mut Tokens) -> Result<Statement, ParserError> {
    tokens.next();
    let name = match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(n),
            ..
        }) => Atom::from(n),
        Some(SpannedToken { token, .. }) => {
            return Err(ParserError::ExpectedToken(
                Token::Identifier("enumfield".to_string()),
                token,
            ));
        }
        None => return Err(ParserError::UnexpectedEOF),
    };
    let mut variants: Vec<Atom<EmptyStaticAtomSet>> = vec![];
    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;
    if let Some(SpannedToken {
        token: Token::Identifier(n),
        span,
    }) = tokens.next()
    {
        variants.push(Atom::from(n));
    }

    while let Some(SpannedToken {
        token: Token::Newline,
        ..
    }) = tokens.next()
    {
        match tokens.next() {
            Some(SpannedToken {
                token: Token::Identifier(n),
                ..
            }) => {
                variants.push(Atom::from(n));
            }
            Some(SpannedToken {
                token: Token::Dedent,
                ..
            }) => {
                break;
            }
            _ => {}
        }
    }

    Ok(Statement::EnumDecl { name, variants })
}
