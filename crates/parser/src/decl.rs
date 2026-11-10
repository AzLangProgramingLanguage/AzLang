use std::rc::Rc;

use crate::{
    ast::{Atom, Expr, Statement},
    binary_op::parse_expression,
    errors::ParserError,
    helpers::expect_token,
    shared_ast::{StringEnum, Type},
    types::parse_type,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_decl(tokens: &mut Tokens, is_mutable: bool) -> Result<Statement, ParserError> {
    tokens.next();
    let mut data_typ = parse_type(tokens)?;

    let name = match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(name),
            span: _,
            ..
        }) => name,
        Some(other) => return Err(ParserError::DeclNameNotFound(other.token)),
        None => return Err(ParserError::DeclNameNotFound(Token::Eof)),
    };
    expect_token(tokens, Token::Assign)?;
    let value = parse_expression(tokens)?;
    match (&value, &data_typ) {
        (Expr::String(_s), Type::String(_d)) => data_typ = Type::String(StringEnum::LiteralString),
        _ => {}
    }

    Ok(Statement::Decl {
        name: Atom::from(name),
        typ: data_typ,
        value: Rc::new(Box::new(value)),
        is_mutable,
    })
}

pub fn is_primite_value_to_type(expr: &Expr) -> Type {
    match expr {
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,
        Expr::String(_) => Type::String(StringEnum::LiteralString),
        _ => Type::Any,
    }
}
