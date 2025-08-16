use crate::{
    lexer::Token,
    parser::{
        ast::{Expr, Type},
        expression::{parse_expression, parse_single_expr},
        structs::parse_structs_init,
        types::parse_type,
    },
};
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;
use std::{borrow::Cow, rc::Rc};

pub fn parse_decl<'a, I>(tokens: &mut PeekMoreIterator<I>, is_mutable: bool) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => Cow::Borrowed(name.as_str()),
        other => return Err(eyre!("Dəyişən adı gözlənilirdi, tapıldı: {:?}", other)),
    };

    let typ = if let Some(Token::Colon) = tokens.peek() {
        tokens.next();
        Some(Rc::new(parse_type(tokens)?))
    } else {
        None
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        other => return Err(eyre!("'=' operatoru gözlənilirdi, tapıldı: {:?}", other)),
    }
    let value_expr;
    if let Some(Token::LBrace) = tokens.peek() {
        let typ_clone = typ.clone().unwrap();

        if let Type::Istifadeci(n, _t) = &*typ_clone {
            tokens.next();
            value_expr = parse_structs_init(tokens, n.clone())?;
        } else {
            return Err(eyre!("Obyekt tipi gözlənilirdi"));
        }
    } else {
        value_expr = parse_expression(tokens)?;
    }

    let value = Box::new(value_expr);

    let expr: Expr<'_> = Expr::Decl {
        name,
        transpiled_name: None,
        typ,
        is_mutable,
        value,
    };
    Ok(expr)
}
