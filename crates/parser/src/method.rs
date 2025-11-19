use crate::errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, Parameter, Type},
    expressions::parse_expression,
    helpers::expect_token,
    types::parse_type,
};
type MethodResultType<'a> = (
    &'a str,
    Vec<Parameter<'a>>,
    Vec<Expr<'a>>,
    Option<Type<'a>>,
    bool,
);
pub fn parse_method<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
) -> Result<MethodResultType<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    expect_token(tokens, Token::Method)?;

    let name = match tokens.next() {
        Some(Token::Identifier(n)) => (*n).as_str(),
        other => {
            return Err(ParserError::MethodNameNotFound(other.unwrap().clone())); /* TODO: using unwrap */
        }
    };

    expect_token(tokens, Token::LParen)?;

    let mut params = Vec::new();
    while let Some(token) = tokens.peek() {
        match token {
            Token::ConstantDecl | Token::MutableDecl | Token::Identifier(_) => {
                // Mutability
                let is_mutable = match tokens.peek() {
                    Some(Token::MutableDecl) => {
                        tokens.next();
                        true
                    }
                    Some(Token::ConstantDecl) => {
                        tokens.next();
                        false
                    }
                    _ => false,
                };

                let param_name = match tokens.next() {
                    Some(Token::Identifier(s)) => (*s).as_str(),
                    other => {
                        return Err(ParserError::ParameterNameNotFound(other.unwrap().clone()));
                    } /* TODO: using unwrap */
                };

                match tokens.peek() {
                    Some(Token::Colon) => {
                        tokens.next();
                    }
                    Some(Token::RParen) => break,
                    other => {
                        return Err(ParserError::ParameterNotExpected(
                            other.unwrap().clone().clone(),
                        )); /* FIXME: Double clone */
                    }
                }
                params.push(Parameter {
                    name: param_name.to_string(),
                    typ: parse_type(tokens)?,
                    is_mutable,
                    is_pointer: false,
                });
            }
            Token::RParen => break,
            Token::Comma => {
                tokens.next();
            }
            other => {
                return Err(ParserError::RParenNotFound(other.clone().clone())); /* FIXME: Double clone */
            }
        }
    }

    expect_token(tokens, Token::RParen)?;
    expect_token(tokens, Token::Colon)?;
    let return_type = Some(parse_type(tokens)?);
    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut body = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::Dedent => {
                tokens.next();
                break;
            }
            Token::Newline => {
                tokens.next();
            }
            Token::Eof => break,
            _ => {
                let expr = parse_expression(tokens)?;
                body.push(expr);
            }
        }
    }

    Ok((name, params, body, return_type, false))
}
