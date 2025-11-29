use std::borrow::Cow;

use crate::{errors::ParserError, typed_ast::TypedExpr};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::ast::Expr;

pub fn parse_enum_decl_core<'a, I, Out>(
    tokens: &mut PeekMoreIterator<I>,
    finish: impl Fn(Cow<'a, str>, Vec<Cow<'a, str>>) -> Out,
) -> Result<Out, ParserError>
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
                tokens.next();
            }
            Some(Token::Identifier(var)) => {
                variants.push(Cow::Borrowed((*var).as_str()));
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
            Some(tok) => {
                return Err(ParserError::UnexpectedToken((*tok).clone()));
            }
            None => return Err(ParserError::UnexpectedEOF),
        }
    }
    Ok(finish(name, variants))
}

pub fn parse_enum_decl<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_enum_decl_core(tokens, |name, variants| Expr::EnumDecl { name, variants })
}

pub fn parse_enum_decl_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_enum_decl_core(tokens, |name, variants| TypedExpr::EnumDecl {
        name,
        variants,
    })
}
