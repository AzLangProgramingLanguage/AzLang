use color_eyre::eyre::Result;
use peekmore::PeekMoreIterator;
use std::borrow::Cow;

use crate::{
    lexer::Token,
    parser::{ast::Expr, expression::parse_single_expr},
    translations::parser_errors::ParserError,
};

pub fn parse_structs_init<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    name: Cow<'a, str>,
) -> Result<Expr<'a>, ParserError>
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
                        return Err(ParserError::StructInitError);
                    }
                };
                match tokens.next() {
                    Some(Token::Colon) => {}
                    _ => {
                        return Err(ParserError::StructInitColonError);
                    }
                }
                let arg_value = parse_single_expr(tokens)?;
                args.push((arg_name, arg_value));
                if let Some(Token::Comma) = tokens.peek() {
                    tokens.next();
                } else {
                    if !matches!(tokens.peek(), Some(Token::RBrace)) {
                        return Err(ParserError::StructInitError);
                    }
                }
            }
        }
    }

    Ok(Expr::StructInit {
        name,
        transpiled_name: None,
        args,
    })
}
