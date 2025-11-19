use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, Type},
        expression::parse_expression,
        structs::parse_structs_init,
        types::parse_type,
    },
};
use color_eyre::eyre::Result;
use peekmore::PeekMoreIterator;
use std::{borrow::Cow, rc::Rc};

pub fn parse_decl<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    is_mutable: bool,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => Cow::Borrowed(name.as_str()),
        other => return Err(ParserError::VariableName(format!("{:?}", other))),
    };
    let mut is_primitive = false;

    let typ = if let Some(Token::Colon) = tokens.peek() {
        tokens.next();
        Some(Rc::new(parse_type(tokens)?))
    } else {
        None
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        other => return Err(ParserError::OperatorError('=', format!("{:?}", other))),
    }
    let value_expr;
    if let Some(Token::LBrace) = tokens.peek() {
        let typ_clone = typ.clone().unwrap();

        if let Type::User(n, _t) = &*typ_clone {
            tokens.next();
            value_expr = parse_structs_init(tokens, n.clone())?;
        } else {
            return Err(ParserError::ObjectTypeNotFound);
        }
    } else {
        value_expr = parse_expression(tokens)?;
        match value_expr {
            Expr::String(..) | Expr::Number(..) | Expr::Bool(..) | Expr::List(..) => {
                is_primitive = true;
            }
            _ => {}
        }
    }

    let value = Box::new(value_expr);

    let expr: Expr<'_> = Expr::Decl {
        name,
        transpiled_name: None,
        is_primitive,
        typ,
        is_mutable,
        value,
    };
    Ok(expr)
}
