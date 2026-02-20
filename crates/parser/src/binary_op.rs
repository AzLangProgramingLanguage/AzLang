use crate::ast::Operation;
use crate::errors::ParserError;
use crate::shared_ast::Type;
use crate::{ast::Expr, expressions::parse_single_expr};
use tokenizer::iterator::{SpannedToken, Tokens};
use tokenizer::tokens::Token;

pub fn parse_expression<'a>(tokens: &mut Tokens) -> Result<Expr<'a>, ParserError> {
    let expr = parse_single_expr(tokens)?;
    match tokens.peek() {
        Some(SpannedToken {
            token: Token::Add, ..
        })
        | Some(SpannedToken {
            token: Token::Subtract,
            ..
        })
        | Some(SpannedToken {
            token: Token::Multiply,
            ..
        })
        | Some(SpannedToken {
            token: Token::Divide,
            ..
        })
        | Some(SpannedToken {
            token: Token::Modulo,
            ..
        })
        | Some(SpannedToken {
            token: Token::Greater,
            ..
        })
        | Some(SpannedToken {
            token: Token::Less, ..
        })
        | Some(SpannedToken {
            token: Token::Equal,
            ..
        })
        | Some(SpannedToken {
            token: Token::NotEqual,
            ..
        })
        | Some(SpannedToken {
            token: Token::Not, ..
        })
        | Some(SpannedToken {
            token: Token::And, ..
        })
        | Some(SpannedToken {
            token: Token::Or, ..
        })
        | Some(SpannedToken {
            token: Token::GreaterEqual,
            ..
        })
        | Some(SpannedToken {
            token: Token::LessEqual,
            ..
        }) => {}
        _ => return Ok(expr),
    }

    parse_binary_op_with_precedence(&expr, tokens, 0)
}

fn parse_binary_op_with_precedence<'a>(
    left: &Expr<'a>,
    tokens: &mut Tokens,
    min_precedence: u8,
) -> Result<Expr<'a>, ParserError> {
    let mut result = Expr::Void;
    loop {
        let op = match tokens.peek() {
            Some(SpannedToken {
                token: Token::Add,
                span,
            }) => Operation::Add,
            Some(SpannedToken {
                token: Token::Subtract,
                span,
                ..
            }) => Operation::Subtract,

            Some(SpannedToken {
                token: Token::Multiply,
                span,
                ..
            }) => Operation::Multiply,

            Some(SpannedToken {
                token: Token::Divide,
                span,
                ..
            }) => Operation::Divide,

            Some(SpannedToken {
                token: Token::Modulo,
                span,
                ..
            }) => Operation::Modulo,

            Some(SpannedToken {
                token: Token::Greater,
                span,
                ..
            }) => Operation::Greater,

            Some(SpannedToken {
                token: Token::Less,
                span,
                ..
            }) => Operation::Less,

            Some(SpannedToken {
                token: Token::Equal,
                span,
                ..
            }) => Operation::Equal,

            Some(SpannedToken {
                token: Token::NotEqual,
                span,
                ..
            }) => Operation::NotEqual,

            Some(SpannedToken {
                token: Token::Not,
                span,
                ..
            }) => Operation::Not,

            Some(SpannedToken {
                token: Token::And,
                span,
                ..
            }) => Operation::And,

            Some(SpannedToken {
                token: Token::Or,
                span,
                ..
            }) => Operation::Or,

            Some(SpannedToken {
                token: Token::GreaterEqual,
                span,
                ..
            }) => Operation::GreaterEqual,

            Some(SpannedToken {
                token: Token::LessEqual,
                span,
                ..
            }) => Operation::LessEqual,

            None | Some(_) => {
                break;
            }
        };
        let precedence = operator_precedence(&op);
        if precedence < min_precedence {
            break;
        }
        tokens.next();
        /*BUG: This line has to be fix.  */
        let right = parse_binary_op_with_precedence(&Expr::Void, tokens, precedence + 1)?;
        result = Expr::BinaryOp {
            left: Box::new(left.clone()),
            right: Box::new(right),
            op,
            return_type: Type::Any,
        };
    }

    Ok(result)
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
