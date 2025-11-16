use std::{borrow::Cow, rc::Rc};

use errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::parser::{
    ast::{Expr, Type},
    expressions::parse_expression,
    struct_init::parse_structs_init,
    types::parse_type,
};

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

    let typ = if let Some(Token::Colon) = tokens.peek() {
        tokens.next();
        Some(Rc::new(parse_type(tokens)?))
    } else {
        None
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        Some(other) => return Err(ParserError::DeclAssignNotFound(other.clone())),
        None => return Err(ParserError::DeclAssignNotFound(Token::Eof)),
    }
    let value_expr;
    if let Some(Token::LBrace) = tokens.peek() {
        let typ_clone = typ.clone().unwrap();

        if let Type::Istifadeci(n) = &*typ_clone {
            tokens.next();
            value_expr = parse_structs_init(tokens, n.clone())?;
        } else {
            return Err(ParserError::ObjectTypeNotExpected(typ_clone)); /* TODO: Dependency Problem */
        }
    } else {
        value_expr = parse_expression(tokens)?;
    }

    let value = Box::new(value_expr);

    let expr: Expr<'_> = Expr::Decl {
        name,
        typ,
        is_mutable,
        value,
    };
    Ok(expr)
}
