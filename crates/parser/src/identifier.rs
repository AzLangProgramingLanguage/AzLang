use std::borrow::Cow;

use crate::{
    ast::Expr,
    errors::ParserError,
    expressions::{parse_expression, parse_single_expr},
    shared_ast::Type,
    struct_init::parse_structs_init,
};
use peekmore::PeekMoreIterator;
use tokenizer::tokens::Token;

pub fn parse_identifier<'a, I>(
    tokens: &mut PeekMoreIterator<I>,
    s: &'a str,
) -> Result<Expr<'a>, ParserError>
where
    I: Iterator<Item = &'a Token>,
{
    let expr = Expr::VariableRef {
        name: Cow::Borrowed(s),
        symbol: None,
    };
    let Some(peek) = tokens.peek() else {
        return Ok(expr);
    };
    match peek {
        Token::ListStart => {
            tokens.next();

            let index_expr = parse_single_expr(tokens)?;

            let Some(token) = tokens.next() else {
                return Err(ParserError::ArrayNotClosed(Token::ListEnd));
            };

            if *token != Token::ListEnd {
                return Err(ParserError::ArrayNotClosed(token.clone()));
            }

            Ok(Expr::Index {
                target: Box::new(expr),
                index: Box::new(index_expr),
                target_type: Type::Any,
            })
        }

        Token::LParen => {
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
    }
}
