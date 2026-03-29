use std::borrow::Cow;

use crate::{
    ast::{Expr, Statement, Symbol},
    binary_op::parse_expression,
    errors::ParserError,
    expressions::parse_single_expr,
    helpers::expect_token,
    shared_ast::Type,
};
use tokenizer::{
    iterator::{SpannedToken, Tokens},
    tokens::Token,
};

pub fn parse_identifier(tokens: &mut Tokens, s: String) -> Result<Expr, ParserError> {
    match tokens.peek() {
        Some(SpannedToken {
            token: Token::LParen,
            span,
        }) => {
            tokens.next();
            let mut args = Vec::new();
            loop {
                match tokens.peek() {
                    Some(SpannedToken {
                        token: Token::RParen,
                        ..
                    }) => {
                        tokens.next();
                        break;
                    }
                    Some(SpannedToken {
                        token: Token::Comma,
                        ..
                    }) => {
                        tokens.next();
                    }
                    None => {
                        return Err(ParserError::RParenNotFound(Token::Eof));
                    }
                    _ => {
                        args.push(parse_single_expr(tokens)?);
                    }
                }
            }
            Ok(Expr::Call {
                target: None,
                name: Box::new(Expr::VariableRef {
                    name: s,
                    symbol: Some(Symbol {
                        is_changed: false,
                        is_mutable: false,
                        is_pointer: false,
                        is_used: true,
                        typ: Type::Function,
                    }),
                }),
                args,
                returned_type: None,
            }) //TODO: Badd Code 
        }
        Some(SpannedToken {
            token: Token::ListStart,
            ..
        }) => {
            tokens.next();
            let index = parse_expression(tokens)?;
            expect_token(tokens, Token::ListEnd)?;
            Ok(Expr::Index {
                target: Box::new(Expr::VariableRef {
                    name: s,
                    symbol: None,
                }),
                index: Box::new(index),
                target_type: Type::Any,
            })
        }
        _ => {
            return Ok(Expr::VariableRef {
                name: s,
                symbol: None,
            });
        }
    }
}
