use crate::{
    ast::{Expr, Operation, Statement},
    binary_op::{parse_expression, parse_statement},
    shared_ast::{BuiltInFunction, Type},
    tests::{TestResult, create_tokens},
};
#[cfg(test)]
use tokenizer::tokens::Token;

#[test]
fn test_binary_op_print() -> TestResult {
    let mut tokens = create_tokens(vec![Token::Number(2), Token::Add, Token::Number(2)]);
    let result = parse_expression(&mut tokens)?;
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            right: Box::new(Expr::Number(2)),
            op: Operation::Add,
            return_type: Type::Any,
        }
    );
    Ok(())
}

#[test]
fn test_multi_add_binary_op() -> TestResult {
    let mut tokens = create_tokens(vec![
        Token::Number(2),
        Token::Add,
        Token::Number(2),
        Token::Add,
        Token::Number(4),
    ]);
    let result = parse_expression(&mut tokens)?;
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Number(2)),
                right: Box::new(Expr::Number(2)),
                op: Operation::Add,
                return_type: Type::Any,
            }),
            right: Box::new(Expr::Number(4)),
            op: Operation::Add,
            return_type: Type::Any,
        }
    );
    Ok(())
}

#[test]
fn test_multiply_add_binary_op() -> TestResult {
    let mut tokens = create_tokens(vec![
        Token::Number(2),
        Token::Add,
        Token::Number(2),
        Token::Multiply,
        Token::Number(4),
    ]);
    let mut tokens2 = create_tokens(vec![
        Token::Number(2),
        Token::Multiply,
        Token::Number(2),
        Token::Add,
        Token::Number(4),
    ]);
    let result = parse_expression(&mut tokens)?;
    let result2 = parse_expression(&mut tokens2)?;
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Number(2)),
                right: Box::new(Expr::Number(4)),
                op: Operation::Multiply,
                return_type: Type::Any,
            }),
            op: Operation::Add,
            return_type: Type::Any,
        }
    );
    assert_eq!(
        result2,
        Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Number(2)),
                right: Box::new(Expr::Number(2)),
                op: Operation::Multiply,
                return_type: Type::Any,
            }),
            right: Box::new(Expr::Number(4)),
            op: Operation::Add,
            return_type: Type::Any,
        }
    );
    Ok(())
}

#[test]
fn test_equal_binary_op() -> TestResult {
    let mut tokens = create_tokens(vec![Token::Number(2), Token::Equal, Token::Number(2)]);
    let mut tokens2 = create_tokens(vec![Token::Number(2), Token::NotEqual, Token::Number(2)]);
    let result = parse_expression(&mut tokens)?;
    let result2 = parse_expression(&mut tokens2)?;
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            right: Box::new(Expr::Number(2)),
            op: Operation::Equal,
            return_type: Type::Any,
        }
    );
    assert_eq!(
        result2,
        Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            right: Box::new(Expr::Number(2)),
            op: Operation::NotEqual,
            return_type: Type::Any,
        }
    );
    Ok(())
}
