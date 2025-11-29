use std::{borrow::Cow, rc::Rc};

use crate::{
    errors::ParserError, parsing_for::parse_expression_typed, shared_ast::Type,
    typed_ast::TypedExpr,
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, expressions::parse_expression, types::parse_type};

fn parse_decl_core<'a, I, Out>(
    tokens: &mut PeekMoreIterator<I>,
    is_mutable: bool,
    parse_expr: impl Fn(&mut PeekMoreIterator<I>) -> Result<Out, ParserError>,
    finish: impl Fn(Cow<'a, str>, Type<'a>, Box<Out>, bool) -> Out,
) -> Result<Out, ParserError>
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
    let value_expr = parse_expr(tokens)?;

    let value = Box::new(value_expr);
    Ok(finish(name, typ, value, is_mutable))
}

pub fn parse_decl<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    is_mutable: bool,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_decl_core(
        tokens,
        is_mutable,
        |tokens| parse_expression(tokens),
        |name, typ, value, is_mutable| Expr::Decl {
            name,
            typ: Some(Rc::new(typ)),
            is_mutable,
            value,
        },
    )
}

pub fn parse_decl_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    is_mutable: bool,
) -> Result<TypedExpr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    parse_decl_core(
        tokens,
        is_mutable,
        |tokens| parse_expression_typed(tokens),
        |name, typ, value, is_mutable| TypedExpr::Decl {
            name,
            typ: Some(Rc::new(typ)),
            is_mutable,
            value,
            transpiled_name: None,
            is_primitive: false,
        },
    )
}
