use crate::{
    ast::{Expr, Parameter},
    errors::ParserError,
    expressions::parse_expression,
    helpers::expect_token,
    shared_ast::Type,
    types::parse_type,
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

pub fn parse_function_def<'a, I>(tokens: &mut PeekMoreIterator<I>) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let name = match tokens.next() {
        Some(Token::Identifier(n)) => (*n).as_str(),
        None => return Err(ParserError::UnexpectedEOF),
        Some(other) => return Err(ParserError::FunctionNameNotFound(other.clone())),
    };

    expect_token(tokens, Token::LParen)?;

    let mut params = Vec::new();

    while let Some(tok) = tokens.peek() {
        match tok {
            Token::ConstantDecl | Token::MutableDecl | Token::Identifier(_) => {
                let is_mutable = matches!(tok, Token::MutableDecl);
                if matches!(tok, Token::MutableDecl | Token::ConstantDecl) {
                    tokens.next();
                }

                let param_name = match tokens.next() {
                    Some(Token::Identifier(s)) => (*s).as_str().to_string(),
                    other => {
                        return Err(ParserError::ParameterNameNotFound(other.unwrap().clone()));
                    }
                };

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
                    None => return Err(ParserError::UnexpectedEOF),
                    Some(other) => {
                        return Err(ParserError::ParameterNotExpected((*other).clone()));
                    }
                }

                params.push(Parameter {
                    name: param_name,
                    typ: param_type,
                    is_mutable,
                    is_pointer: false,
                });
            }

            Token::Comma => {
                tokens.next();
            }

            Token::RParen => break,

            other => return Err(ParserError::RParenNotFound((*other).clone())),
        }
    }

    expect_token(tokens, Token::RParen)?;
    expect_token(tokens, Token::Colon)?;

    let return_type = Some(parse_type(tokens)?);

    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut body = Vec::new();

    while let Some(tok) = tokens.peek() {
        match tok {
            Token::Dedent => {
                tokens.next();
                break;
            }
            Token::Newline => {
                tokens.next();
            }
            Token::Eof => break,
            _ => {
                body.push(parse_expression(tokens)?);
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
