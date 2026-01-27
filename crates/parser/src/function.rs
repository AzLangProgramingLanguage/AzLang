use crate::{
    ast::{Expr, Parameter},
    errors::ParserError,
    expressions::parse_expression,
    helpers::expect_token,
    shared_ast::Type,
    types::parse_type,
};
use tokenizer::{iterator::{SpannedToken, Tokens}, tokens::Token};

pub fn parse_function_def<'a>(tokens: &mut Tokens) -> Result<Expr<'a>, ParserError>
{
    let name = match tokens.next() {
        Some(SpannedToken { token: Token::Identifier(n), .. }) => n,
        None => return Err(ParserError::UnexpectedEOF),
        Some(SpannedToken { token: other, .. }) => return Err(ParserError::FunctionNameNotFound(other.clone())),
    };

    expect_token(tokens, Token::LParen)?;

    let mut params = Vec::new();

    while let Some(tok) = tokens.peek() {
        match &tok.token {
            Token::ConstantDecl | Token::MutableDecl | Token::Identifier(_) => {
                let is_mutable = matches!(tok.token, Token::MutableDecl);
                if matches!(tok.token, Token::MutableDecl | Token::ConstantDecl) {
                    tokens.next();
                }

                let param_name = match tokens.next() {
                    Some(SpannedToken { token: Token::Identifier(s), .. }) => s,
                    other => {
                        return Err(ParserError::ParameterNameNotFound(other.unwrap().token));
                    }
                };

                let mut param_type = Type::Any;

                match tokens.peek() {
                    Some(SpannedToken { token: Token::Comma, .. }) => {
                        tokens.next();
                    }
                    Some(SpannedToken { token: Token::Colon, .. }) => {
                        tokens.next();
                        param_type = parse_type(tokens)?;
                    }
                    Some(SpannedToken { token: Token::RParen, .. }) => break,
                    None => return Err(ParserError::UnexpectedEOF),
                    Some(SpannedToken { token: other, .. }) => {
                        return Err(ParserError::ParameterNotExpected(other.clone()));
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

            other => return Err(ParserError::RParenNotFound(other.clone())),
        }
    }
    expect_token(tokens, Token::RParen)?;
    expect_token(tokens, Token::Colon)?;

    let return_type = Some(parse_type(tokens)?);

    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::Indent)?;

    let mut body = Vec::new();

    while let Some(tok) = tokens.peek() {
        match tok.token {
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
