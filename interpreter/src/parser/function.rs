use errors::ParserError;
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

use crate::parser::{
    ast::{Expr, Parameter, Type},
    expressions::parse_expression,
    helpers::expect_token,
    types::parse_type,
};

pub fn parse_function_def<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(name)) => (*name).as_str(),
        other => return Err(ParserError::FunctionNameNotFound(other.unwrap().clone())), /* TODO: using unwrap */
    };

    expect_token(tokens, Token::LParen)?;

    let mut params = Vec::new();

    while let Some(token) = tokens.peek() {
        match token {
            Token::ConstantDecl | Token::MutableDecl | Token::Identifier(_) => {
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

                // Parametr adı
                let param_name = match tokens.next() {
                    Some(Token::Identifier(s)) => (*s).as_str(),
                    other => {
                        return Err(ParserError::ParameterNameNotFound(other.unwrap().clone()));
                    } /* TODO: using unwrap */
                };

                // Tip varsa oxu
                let mut param_type = Type::Any;

                match tokens.peek() {
                    Some(Token::Comma) => {
                        tokens.next();
                    }
                    Some(Token::Colon) => {
                        tokens.next();
                        param_type = parse_type(tokens)?;
                    }
                    Some(Token::RParen) => break,
                    other => {
                        return Err(ParserError::ParameterNotExpected(
                            other.unwrap().clone().clone(), /* FIXME: Double clone */
                        ));
                    }
                }
                params.push(Parameter {
                    name: param_name.to_string(),
                    typ: param_type,
                    is_mutable,
                    is_pointer: false,
                });
            }
            Token::Comma => {
                tokens.next();
            }
            Token::RParen => break,
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

    Ok(Expr::FunctionDef {
        name,
        params,
        body,
        return_type,
    })
}
