use std::{borrow::Cow, rc::Rc};

use crate::errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, expressions::parse_expression, types::parse_type};

pub fn parse_decl<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    is_mutable: bool,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let typ = parse_type(tokens)?;

    let name = match tokens.next() {
        Some(Token::Identifier(name)) => Cow::Borrowed(name.as_str()),
        Some(other) => return Err(ParserError::DeclNameNotFound(other.clone())),
        None => return Err(ParserError::DeclNameNotFound(Token::Eof)),
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        Some(other) => return Err(ParserError::DeclAssignNotFound(other.clone())),
        None => return Err(ParserError::DeclAssignNotFound(Token::Eof)),
    }
    let value_expr = parse_expression(tokens)?;

    let value = Box::new(value_expr);
    Ok(Expr::Decl {
        name,
        typ: Some(Rc::new(typ)),
        value,
        is_mutable,
    })
}
