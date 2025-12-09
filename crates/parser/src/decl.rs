use std::{borrow::Cow, rc::Rc};

use crate::{errors::ParserError, shared_ast::Type};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::{self, Token};

use crate::{ast::Expr, expressions::parse_expression, types::parse_type};

pub fn parse_decl<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    is_mutable: bool,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let mut typ = Type::Any;
    let name: Cow<'a, str>;
    match tokens.peek_nth(1) {
        Some(Token::Operator(op)) if op == "=" => {
            name = match tokens.next() {
                Some(Token::Identifier(name)) => Cow::Borrowed(name.as_str()),
                Some(other) => return Err(ParserError::DeclNameNotFound(other.clone())),
                None => return Err(ParserError::DeclNameNotFound(Token::Eof)),
            };
        }
        Some(Token::Identifier(s)) => {
            typ = parse_type(tokens)?;
            name = Cow::Borrowed(s.as_str());
            tokens.next();
        }
        Some(other) => return Err(ParserError::DeclNameNotFound((*other).clone())),
        None => return Err(ParserError::DeclAssignNotFound(Token::Eof)),
    }

    tokens.next();
    let value_expr = parse_expression(tokens)?;
    match tokens.peek() {
        Some(Token::Newline) => {}
        Some(other) => return Err(ParserError::UnexpectedToken((*other).clone())),
        None => return Err(ParserError::UnexpectedEOF),
    }
    let value = Box::new(value_expr);
    Ok(Expr::Decl {
        name,
        typ: Rc::new(typ),
        value,
        is_mutable,
    })
}

pub fn is_primite_value_to_type<'a>(expr: &Expr<'a>) -> Type<'a> {
    match expr {
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,
        Expr::String(_) => Type::LiteralString,
        _ => Type::Any,
    }
}
