use core::panic;

use crate::assign::parse_assign;
use crate::ast::{Operation, Statement};
use crate::condition::parse_if_expr;
use crate::decl::parse_decl;
use crate::errors::ParserError;
use crate::r#loop::parse_loop;
use crate::shared_ast::Type;
use crate::{ast::Expr, expressions::parse_single_expr};
use tokenizer::iterator::{SpannedToken, Tokens};
use tokenizer::tokens::Token;

pub fn parse_statement<'a>(tokens: &mut Tokens) -> Result<Statement, ParserError> {
    match tokens.peek() {
        Some(SpannedToken {
            token: Token::Conditional,
            ..
        }) => return Ok(parse_if_expr(tokens)?),
        Some(SpannedToken {
            token: Token::Identifier(s),
            ..
        }) if tokens.peek().is_some_and(|t| t.token == Token::Assign) => {
            return Ok(parse_assign(tokens, s.to_string())?);
        }

        Some(SpannedToken {
            token: Token::Loop, ..
        }) => return Ok(parse_loop(tokens)?),
        Some(SpannedToken {
            token: Token::ConstantDecl,
            ..
        }) => return Ok(parse_decl(tokens, false)?),
        Some(SpannedToken {
            token: Token::MutableDecl,
            ..
        }) => return Ok(parse_decl(tokens, true)?),
        _ => return Ok(Statement::Expr(parse_expression(tokens)?)),
    }
}

pub fn parse_expression<'a>(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let expr = parse_single_expr(tokens)?;

    Ok(parse_binary_op_with_precedence(expr, tokens, 0)?)
}

fn parse_binary_op_with_precedence<'a>(
    mut left: Expr,
    tokens: &mut Tokens,
    min_precedence: u8,
) -> Result<Expr, ParserError> {
    match tokens.peek() {
        Some(SpannedToken {
            token: Token::LParen,
            ..
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
            return Ok(Expr::Call {
                target: None,
                name: Box::new(left),
                args,
                returned_type: Some(Type::Void),
            });
        }

        _ => {}
    }
    loop {
        let op = match tokens.peek() {
            Some(SpannedToken {
                token: Token::Add, ..
            }) => Operation::Add,
            Some(SpannedToken {
                token: Token::Subtract,
                ..
            }) => Operation::Subtract,
            Some(SpannedToken {
                token: Token::Multiply,
                ..
            }) => Operation::Multiply,
            Some(SpannedToken {
                token: Token::Divide,
                ..
            }) => Operation::Divide,
            Some(SpannedToken {
                token: Token::Modulo,
                ..
            }) => Operation::Modulo,
            Some(SpannedToken {
                token: Token::Greater,
                ..
            }) => Operation::Greater,
            Some(SpannedToken {
                token: Token::Less, ..
            }) => Operation::Less,
            Some(SpannedToken {
                token: Token::Equal,
                ..
            }) => Operation::Equal,
            Some(SpannedToken {
                token: Token::NotEqual,
                ..
            }) => Operation::NotEqual,
            Some(SpannedToken {
                token: Token::And, ..
            }) => Operation::And,
            Some(SpannedToken {
                token: Token::Or, ..
            }) => Operation::Or,
            Some(SpannedToken {
                token: Token::GreaterEqual,
                ..
            }) => Operation::GreaterEqual,
            Some(SpannedToken {
                token: Token::LessEqual,
                ..
            }) => Operation::LessEqual,
            _ => break,
        };
        tokens.next();

        let precedence = operator_precedence(&op);
        if precedence < min_precedence {
            break;
        }

        let rhs = parse_single_expr(tokens)?;

        let right = parse_binary_op_with_precedence(rhs, tokens, precedence + 1)?;
        left = Expr::BinaryOp {
            left: Box::new(left),
            right: Box::new(right),
            op,
            return_type: Type::Any,
        };
    }

    Ok(left)
}
fn operator_precedence(op: &Operation) -> u8 {
    match op {
        Operation::Multiply | Operation::Divide | Operation::Modulo => 3,
        Operation::Add | Operation::Subtract => 2,
        Operation::Equal
        | Operation::NotEqual
        | Operation::Less
        | Operation::Greater
        | Operation::LessEqual
        | Operation::GreaterEqual => 1,
        _ => 0,
    }
}
