use crate::{
    lexer::Token,
    parser::{ast::Expr, expression::parse_single_expr, types::parse_type},
};
use color_eyre::eyre::{Result, eyre};
use peekmore::PeekMoreIterator;
use std::borrow::Cow;

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
        Some(parse_type(tokens)?)
    } else {
        None
    };

    match tokens.next() {
        Some(Token::Operator(op)) if op == "=" => {}
        other => return Err(eyre!("'=' operatoru gözlənilirdi, tapıldı: {:?}", other)),
    }

    let value_expr = parse_single_expr(tokens)?;

    let value = Box::new(value_expr);

    tokens.next();

    Ok(Expr::Decl {
        name,
        typ,
        is_mutable,
        value,
    })
}
