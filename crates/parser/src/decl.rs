use std::{borrow::Cow, rc::Rc};

use crate::{errors::ParserError, shared_ast::Type};
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
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => Cow::Borrowed(name.as_str()),
        Some(other) => return Err(ParserError::DeclNameNotFound(other.clone())),
        None => return Err(ParserError::DeclNameNotFound(Token::Eof)),
    };

    let typ = match tokens.next() {
        Some(Token::Colon) => parse_type(tokens)?,
        Some(_) => Type::Any,
        None => Type::Any,
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        Some(other) => return Err(ParserError::DeclAssignNotFound(other.clone())),
        None => return Err(ParserError::DeclAssignNotFound(Token::Eof)),
    }

    let value_expr = parse_expression(tokens)?;

    let value = Box::new(value_expr);

    let expr: Expr<'_> = Expr::Decl {
        name,
        typ: Some(Rc::new(typ)),
        is_mutable,
        value,
    };
    Ok(expr)
}
