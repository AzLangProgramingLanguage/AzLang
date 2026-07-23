use crate::assign::parse_assign;
use crate::ast::{Expr, Operation, Statement};
use crate::condition::parse_if_expr;
use crate::decl::parse_decl;
use crate::errors::ParserError;
use crate::expressions::parse_single_expr;
use crate::function::{parse_external_function_def, parse_function_def, parse_link_directive};
use crate::r#loop::parse_loop;
use crate::r#while_loop::parse_while_loop;
use tokenizer::iterator::{SpannedToken, Tokens};
use tokenizer::tokens::Token;

pub fn parse_statement(tokens: &mut Tokens) -> Result<Statement, ParserError> {
    match tokens.peek() {
        Some(SpannedToken {
            token: Token::Conditional,
            ..
        }) => parse_if_expr(tokens),
        Some(SpannedToken {
            token: Token::Identifier(s),
            ..
        }) if tokens.peek_nth(1).is_some_and(|t| t.token == Token::Assign) => {
            parse_assign(tokens, s.to_string())
        }
        Some(SpannedToken {
            token: Token::At, ..
        }) => match tokens.peek_nth(1).map(|t| &t.token) {
            Some(Token::Identifier(s)) if s == "link" => {
                tokens.next();
                let link_name = parse_link_directive(tokens)?;
                match tokens.peek().map(|t| &t.token) {
                    Some(Token::At) => parse_external_function_def(tokens, Some(link_name)),
                    _ => Err(ParserError::ExpectedToken(
                        Token::At,
                        tokens.peek().map(|t| t.token.clone()).unwrap_or(Token::Eof),
                    )),
                }
            }
            _ => parse_external_function_def(tokens, None),
        },

        Some(SpannedToken {
            token: Token::While,
            ..
        }) => parse_while_loop(tokens),
        Some(SpannedToken {
            token: Token::FunctionDef,
            ..
        }) => parse_function_def(tokens),

        Some(SpannedToken {
            token: Token::Loop, ..
        }) => parse_loop(tokens),
        Some(SpannedToken {
            token: Token::ConstantDecl,
            ..
        }) => parse_decl(tokens, false),
        Some(SpannedToken {
            token: Token::MutableDecl,
            ..
        }) => parse_decl(tokens, true),
        _ => Ok(Statement::Expr(parse_expression(tokens)?)),
    }
}

pub fn parse_expression(tokens: &mut Tokens) -> Result<Expr, ParserError> {
    let expr = parse_single_expr(tokens)?;

    parse_binary_op_with_precedence(expr, tokens, 0)
}

fn parse_binary_op_with_precedence(
    mut left: Expr,
    tokens: &mut Tokens,
    min_precedence: u8,
) -> Result<Expr, ParserError> {
    if let Some(SpannedToken {
        token: Token::LParen,
        ..
    }) = tokens.peek()
    {
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
                    args.push(parse_expression(tokens)?);
                }
            }
        }
        return Ok(Expr::Call {
            target: None,
            name: Box::new(left),
            args,
        });
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

        let precedence = operator_precedence(&op);
        if precedence < min_precedence {
            break;
        }

        tokens.next();
        let rhs = parse_single_expr(tokens)?;

        let right = parse_binary_op_with_precedence(rhs, tokens, precedence + 1)?;
        left = Expr::BinaryOp {
            left: Box::new(left),
            right: Box::new(right),
            op,
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
