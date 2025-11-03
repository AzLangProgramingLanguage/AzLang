use crate::{
    lexer::Token,
    parser::ast::{EnumDecl, Expr},
    translations::parser_errors::ParserError,
};
use color_eyre::eyre::Result;
use peekmore::PeekMoreIterator;
use std::{borrow::Cow, f64::EPSILON, string::ParseError};

pub fn parse_enum_decl<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    tokens.next();
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => Cow::Borrowed((*name).as_str()),
        other => {
            return Err(ParserError::UnionIdentifierNotFound(format!("{:?}", other)));
        }
    };

    // 2. Yeni sətrə keçirik
    match tokens.next() {
        Some(Token::Newline) => {}
        other => {
            return Err(ParserError::EnumNewlineError(format!("{:?}", other)));
        }
    }

    let mut variants = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::Indent) => {
                tokens.next(); // indent keçirik
            }
            Some(Token::Identifier(var)) => {
                variants.push(Cow::Borrowed((*var).as_str()));
                tokens.next(); // consume name
            }
            Some(Token::Newline) => {
                tokens.next(); // boş sətir, skip
            }
            Some(Token::Dedent) | Some(Token::End) => {
                tokens.next(); // block bitdi
                break;
            }
            Some(Token::Eof) => break,
            Some(tok) => {
                return Err(ParserError::EnumVariantNotFound(format!("{:?}", tok)));
            }
            None => return Err(ParserError::Eof),
        }
    }
    Ok(Expr::EnumDecl(EnumDecl { name, variants }))
}
