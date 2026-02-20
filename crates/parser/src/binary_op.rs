use crate::ast::Operation;
use crate::errors::ParserError;
use crate::shared_ast::Type;
use crate::{ast::Expr, expressions::parse_single_expr};
use tokenizer::iterator::{SpannedToken, Tokens};
use tokenizer::tokens::Token;

pub fn parse_expression<'a>(tokens: &mut Tokens) -> Result<Expr<'a>, ParserError> {
    let expr = parse_single_expr(tokens)?;
    parse_binary_op_with_precedence(expr, tokens, 0)
}

fn parse_binary_op_with_precedence<'a>(
    mut left: Expr<'a>,
    tokens: &mut Tokens,
    min_precedence: u8,
) -> Result<Expr<'a>, ParserError> {
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
