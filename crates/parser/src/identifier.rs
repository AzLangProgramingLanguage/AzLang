use std::borrow::Cow;

use crate::{
    ast::Expr,
    errors::ParserError,
    expressions::{parse_expression, parse_single_expr},
    shared_ast::Type,
};
use tokenizer::{iterator::{SpannedToken, Tokens}, tokens::Token};

pub fn parse_identifier<'a>(
    tokens: &mut Tokens,
    s: String,
) -> Result<Expr<'a>, ParserError>
{
 
    match tokens.peek(){ 
        Some(SpannedToken { token: Token::Assign, .. }) => {
            tokens.next();
            let value = parse_expression(tokens)?;

            Ok(Expr::Assignment {
                name: s.into(),
                value: Box::new(value),
                symbol: None,
            })
        }
        Some(SpannedToken { token: Token::LParen, .. }) => {
            tokens.next();
            let mut args = Vec::new();
            loop {
                match tokens.peek() {
                    Some(SpannedToken { token: Token::RParen, .. }) => {
                        tokens.next();
                        break;
                    }
                    Some(SpannedToken { token: Token::Comma, .. }) => {
                        tokens.next();
                    }
                    None => return Err(ParserError::RParenNotFound(Token::Eof)),
                    _ => {
                        args.push(parse_single_expr(tokens)?);
                    }
                }
            }

            Ok(Expr::Call {
                target: None,
                name: s,
                args,
                returned_type: Some(Type::Void),
            })
        }
        _ => return Ok(Expr::VariableRef {
            name: Cow::Owned(s),
            symbol: None,
        })
    }
 
   /*  match peek {
        SpannedToken { token: Token::ListStart, .. } => {
            tokens.next();

            let index_expr = parse_single_expr(tokens)?;

            let Some(token) = tokens.next() else {
                return Err(ParserError::ArrayNotClosed(Token::ListEnd));
            };

            if token.token != Token::ListEnd {
                return Err(ParserError::ArrayNotClosed(token.token));
            }

            Ok(Expr::Index {
                target: Box::new(expr),
                index: Box::new(index_expr),
                target_type: Type::Any,
            })
        }

        SpannedToken { token: Token::LParen, .. } => {
            tokens.next();

            let mut args = Vec::new();
            loop {
                match tokens.peek() {
                    Some(Token::RParen) => {
                        tokens.next();
                        break;
                    }
                    Some(Token::Comma) => {
                        tokens.next();
                    }
                    None => return Err(ParserError::RParenNotFound(Token::Eof)),
                    _ => {
                        args.push(parse_expression(tokens)?);
                    }
                }
            }

            Ok(Expr::Call {
                target: None,
                name: s,
                args,
                returned_type: Some(Type::Void),
            })
        }

        Token::Operator(op) if op == "=" => {
            tokens.next();
            let value = parse_expression(tokens)?;

            Ok(Expr::Assignment {
                name: s.into(),
                value: Box::new(value),
                symbol: None,
            })
        }

        Token::Dot => {
            tokens.next(); // '.'

            let field = match tokens.next() {
                Some(Token::Identifier(n)) => (*n).as_str(),
                Some(other) => return Err(ParserError::MethodNameNotFound(other.clone())),
                None => return Err(ParserError::MethodNameNotFound(Token::Eof)),
            };

            // method call
            if let Some(Token::LParen) = tokens.peek() {
                tokens.next(); // '('
                let mut args = Vec::new();

                while let Some(tok) = tokens.peek() {
                    match tok {
                        Token::RParen => {
                            tokens.next();
                            break;
                        }
                        Token::Comma => {
                            tokens.next();
                        }
                        _ => args.push(parse_single_expr(tokens)?),
                    }
                }

                return Ok(Expr::Call {
                    target: Some(Box::new(expr)),
                    name: field,
                    args,
                    returned_type: Some(Type::Any),
                });
            }

            Ok(Expr::Index {
                target: Box::new(expr),
                index: Box::new(Expr::String(field)),
                target_type: Type::Any,
            })
        }

        Token::LBrace => {
            tokens.next();
            parse_structs_init(tokens, Cow::Borrowed(s))
        }

        _ => Ok(expr),
    } */
}
