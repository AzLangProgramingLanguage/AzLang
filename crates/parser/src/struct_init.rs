use std::borrow::Cow;

use crate::{errors::ParserError, parsing_for::parse_single_expr_typed, typed_ast::TypedExpr};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{ast::Expr, expressions::parse_single_expr};

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
                        return Err(ParserError::StructInitArgSeparatorNotFound);
                    }
                };
                match tokens.next() {
                    Some(Token::Colon) => {}
                    Some(other) => {
                        return Err(ParserError::StructInitArgNotExpected(other.clone()));
                    }
                    None => {
                        return Err(ParserError::StructInitArgNotExpected(Token::Eof));
                    }
                }
                let arg_value = parse_single_expr(tokens)?;
                args.push((arg_name, arg_value));
                if let Some(Token::Comma) = tokens.peek() {
                    tokens.next();
                } else {
                    if !matches!(tokens.peek(), Some(Token::RBrace)) {
                        return Err(ParserError::StructInitArgSeparatorNotFound);
                    }
                }
            }
        }
    }

    Ok(Expr::StructInit { name, args })
}

pub fn parse_structs_init_typed<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    name: Cow<'a, str>,
) -> Result<TypedExpr<'a>, ParserError>
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
                        return Err(ParserError::StructInitArgSeparatorNotFound);
                    }
                };
                match tokens.next() {
                    Some(Token::Colon) => {}
                    Some(other) => {
                        return Err(ParserError::StructInitArgNotExpected(other.clone()));
                    }
                    None => {
                        return Err(ParserError::StructInitArgNotExpected(Token::Eof));
                    }
                }
                let arg_value: TypedExpr<'a> = parse_single_expr_typed(tokens)?;
                args.push((arg_name, arg_value));
                if let Some(Token::Comma) = tokens.peek() {
                    tokens.next();
                } else {
                    if !matches!(tokens.peek(), Some(Token::RBrace)) {
                        return Err(ParserError::StructInitArgSeparatorNotFound);
                    }
                }
            }
        }
    }

    Ok(TypedExpr::StructInit {
        name,
        args,
        transpiled_name: None,
    })
}
