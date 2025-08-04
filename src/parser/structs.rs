use color_eyre::eyre::{eyre, Result};
use peekmore::PeekMoreIterator;
use std::borrow::Cow;

use crate::{
    lexer::Token,
    parser::{ast::Expr, expression::parse_single_expr},
};

pub fn parse_structs_init<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    name: Cow<'a, str>,
) -> Result<Expr<'a>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut args = Vec::new();
    loop {
        match tokens.peek() {
            Some(Token::RBrace) => {
                tokens.next();
                break;
            }
            None => break,
            _ => {
                let arg_name = match tokens.next() {
                    Some(Token::Identifier(s)) => s.as_str(),
                    _ => {
                        return Err(eyre!(
                            "Struct init argümentləri arasında ',' və ya '}}' gözlənilirdi"
                        ));
                    }
                };
                match tokens.next() {
                    Some(Token::Colon) => {}
                    _ => {
                        return Err(eyre!("Struct init argümentləri arasında ':' gözlənilirdi"));
                    }
                }
                let arg_value = parse_single_expr(tokens)?;
                args.push((arg_name, arg_value));
                if let Some(Token::Comma) = tokens.peek() {
                    tokens.next();
                } else {
                    if !matches!(tokens.peek(), Some(Token::RBrace)) {
                        return Err(eyre!(
                            "Struct init argümentləri arasında ',' və ya '}}' gözlənilirdi"
                        ));
                    }
                }
            }
        }
    }

    Ok(Expr::StructInit { name, args })
}
