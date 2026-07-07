use crate::{
    ast::{Atom, Expr, Parameter, Statement},
    binary_op::parse_statement,
    errors::ParserError,
    helpers::expect_token,
    shared_ast::Type,
    types::parse_type,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_link_directive(tokens: &mut Tokens) -> Result<Atom, ParserError> {
    match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(ref s),
            ..
        }) if s == "link" => {}
        Some(SpannedToken { token: other, .. }) => {
            return Err(ParserError::ExpectedToken(
                Token::Identifier("link".into()),
                other,
            ));
        }
        None => return Err(ParserError::UnexpectedEOF),
    }
    expect_token(tokens, Token::LParen)?;
    let lib_name = match tokens.next() {
        Some(SpannedToken {
            token: Token::StringLiteral(s),
            ..
        }) => s,
        Some(SpannedToken { token: other, .. }) => {
            return Err(ParserError::ExpectedToken(
                Token::StringLiteral(String::new()),
                other,
            ));
        }
        None => return Err(ParserError::UnexpectedEOF),
    };
    expect_token(tokens, Token::RParen)?;
    expect_token(tokens, Token::Newline)?;
    Ok(Atom::from(lib_name))
}

pub fn parse_external_function_def(
    tokens: &mut Tokens,
    link_name: Option<Atom>,
) -> Result<Statement, ParserError> {
    tokens.next();
    match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(ref s),
            ..
        }) if s == "external" => {}
        Some(SpannedToken { token: other, .. }) => {
            return Err(ParserError::ExpectedToken(
                Token::Identifier("external".into()),
                other,
            ));
        }
        None => return Err(ParserError::UnexpectedEOF),
    }
    expect_token(tokens, Token::LParen)?;
    let library = match tokens.next() {
        Some(SpannedToken {
            token: Token::StringLiteral(s),
            ..
        }) => s,
        Some(SpannedToken { token: other, .. }) => {
            return Err(ParserError::ExpectedToken(
                Token::StringLiteral(String::new()),
                other,
            ));
        }
        None => return Err(ParserError::UnexpectedEOF),
    };
    expect_token(tokens, Token::Comma)?;
    let symbol = match tokens.next() {
        Some(SpannedToken {
            token: Token::StringLiteral(s),
            ..
        }) => s,
        Some(SpannedToken { token: other, .. }) => {
            return Err(ParserError::ExpectedToken(
                Token::StringLiteral(String::new()),
                other,
            ));
        }
        None => return Err(ParserError::UnexpectedEOF),
    };
    expect_token(tokens, Token::RParen)?;
    expect_token(tokens, Token::Newline)?;
    expect_token(tokens, Token::FunctionDef)?;

    let name = match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(n),
            ..
        }) => n,
        None => return Err(ParserError::UnexpectedEOF),
        Some(SpannedToken { token: other, .. }) => {
            return Err(ParserError::FunctionNameNotFound(other));
        }
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
                let param_type = param_typ(tokens, is_mutable)?;
                let param_name = match tokens.next() {
                    Some(SpannedToken {
                        token: Token::Identifier(s),
                        ..
                    }) => s,
                    other => {
                        return Err(ParserError::ParameterNameNotFound(other.unwrap().token));
                    }
                };
                params.push(Parameter {
                    name: Atom::from(param_name),
                    typ: param_type,
                    is_pointer: is_mutable,
                });
                match tokens.peek() {
                    Some(SpannedToken {
                        token: Token::Comma,
                        ..
                    }) => {
                        tokens.next();
                    }
                    Some(SpannedToken {
                        token: Token::RParen,
                        ..
                    }) => break,
                    None => return Err(ParserError::UnexpectedEOF),
                    Some(SpannedToken { token: other, .. }) => {
                        return Err(ParserError::ParameterNotExpected(other.clone()));
                    }
                }
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
    let return_type = parse_type(tokens)?;
    expect_token(tokens, Token::Newline)?;

    Ok(Statement::ExternalFunctionDef {
        name: Atom::from(name),
        return_typ: return_type,
        params,
        library: Atom::from(library),
        symbol: Atom::from(symbol),
        link_name,
    })
}

pub fn parse_function_def(tokens: &mut Tokens) -> Result<Statement, ParserError> {
    tokens.next();
    let name = match tokens.next() {
        Some(SpannedToken {
            token: Token::Identifier(n),
            ..
        }) => n,
        None => return Err(ParserError::UnexpectedEOF),
        Some(SpannedToken { token: other, .. }) => {
            return Err(ParserError::FunctionNameNotFound(other.clone()));
        }
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
                let param_typ = param_typ(tokens, is_mutable)?;

                let param_name = match tokens.next() {
                    Some(SpannedToken {
                        token: Token::Identifier(s),
                        ..
                    }) => s,
                    other => {
                        return Err(ParserError::ParameterNameNotFound(other.unwrap().token));
                    }
                };
                params.push(Parameter {
                    name: Atom::from(param_name),
                    typ: param_typ,
                    is_pointer: is_mutable,
                });
                match tokens.peek() {
                    Some(SpannedToken {
                        token: Token::Comma,
                        ..
                    }) => {
                        tokens.next();
                    }
                    Some(SpannedToken {
                        token: Token::RParen,
                        ..
                    }) => break,
                    None => return Err(ParserError::UnexpectedEOF),
                    Some(SpannedToken { token: other, .. }) => {
                        return Err(ParserError::ParameterNotExpected(other.clone()));
                    }
                }
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

    let return_type = parse_type(tokens)?;

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
                body.push(parse_statement(tokens)?);
            }
        }
    }
    Ok(Statement::FunctionDef {
        name: Atom::from(name),
        return_typ: return_type,
        body,
        params,
    })
}
fn param_typ(tokens: &mut Tokens, is_mutable: bool) -> Result<Type, ParserError> {
    let mut param_type = parse_type(tokens)?;
    match (&param_type, &is_mutable) {
        (Type::String(S), false) => {
            param_type = Type::String(crate::shared_ast::StringEnum::LiteralConstString);
        }
        _ => {}
    }
    Ok(param_type)
}
