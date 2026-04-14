use std::{borrow::Cow, rc::Rc};

use crate::{
    ast::Statement,
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

use crate::ast::Expr;

pub fn parse_decl<'a>(tokens: &mut Tokens, is_mutable: bool) -> Result<Statement, ParserError> {
    tokens.next();
    let data_typ = parse_type(tokens)?;

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

    Ok(Statement::Decl {
        name: name.into(),
        typ: Rc::new(data_typ),
        value: Box::new(value),
        is_mutable,
    })
}

pub fn is_primite_value_to_type<'a>(expr: &Expr, is_mutable: bool) -> Type {
    match expr {
        Expr::Number(_) => Type::Integer,
        Expr::Float(_) => Type::Float,
        Expr::Bool(_) => Type::Bool,
        Expr::Char(_) => Type::Char,
        Expr::String(_) if is_mutable => Type::String(StringEnum::LiteralString),
        Expr::String(_) => Type::String(StringEnum::LiteralConstString),
        _ => Type::Any,
    }
}
