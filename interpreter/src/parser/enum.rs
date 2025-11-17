use std::borrow::Cow;

use errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::parser::ast::{EnumDecl, Expr};

pub fn parse_enum_decl<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => Cow::Borrowed((*name).as_str()),
        other => {
            return Err(ParserError::EnumDeclNameNotFound(
                other.unwrap_or(&Token::Eof).clone(),
            ));
        }
    };

    // 2. Yeni sətrə keçirik
    match tokens.next() {
        Some(Token::Newline) => {}
        other => {
            return Err(ParserError::EnumNewLineNotFound(
                other.unwrap_or(&Token::Eof).clone(),
            ));
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
                return Err(ParserError::UnexpectedToken((*tok).clone()));
            }
            None => return Err(ParserError::UnexpectedEOF),
        }
    }
    Ok(Expr::EnumDecl(EnumDecl { name, variants }))
}
