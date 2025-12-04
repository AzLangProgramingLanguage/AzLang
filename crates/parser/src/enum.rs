use std::borrow::Cow;

use crate::{ast::Expr, errors::ParserError};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

pub fn parse_enum_decl<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(n)) => Cow::Borrowed(n.as_str()),
        other => {
            return Err(ParserError::EnumDeclNameNotFound(
                other.unwrap_or(&Token::Eof).clone(),
            ));
        }
    };

    match tokens.next() {
        Some(Token::Newline) => {}
        other => {
            return Err(ParserError::EnumNewLineNotFound(
                other.unwrap_or(&Token::Eof).clone(),
            ));
        }
    }

    // --- Variants ---
    let mut variants = Vec::new();

    loop {
        match tokens.peek() {
            Some(Token::Indent) => {
                tokens.next();
            }
            Some(Token::Identifier(var)) => {
                variants.push(Cow::Borrowed(var.as_str()));
                tokens.next();
            }
            Some(Token::Newline) => {
                tokens.next();
            }
            Some(Token::Dedent) | Some(Token::End) => {
                tokens.next();
                break;
            }
            Some(Token::Eof) => break,
            Some(tok) => return Err(ParserError::UnexpectedToken((*tok).clone())),
            None => return Err(ParserError::UnexpectedEOF),
        }
    }

    Ok(Expr::EnumDecl { name, variants })
}
