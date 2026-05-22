#[cfg(test)]
use std::{result, vec};

use tokenizer::tokens::Token;

use crate::{
    ast::{Expr, Statement},
    binary_op::{parse_expression, parse_statement},
    shared_ast::{BuiltInFunction, Type},
    tests::create_tokens,
};

#[test]
fn test_binary_op_print() {
    let mut tokens = create_tokens(vec![Token::Number(2), Token::Add, Token::Number(2)]);
    let result = parse_expression(&mut tokens).expect("parse edilemedi");
    let expected = Expr::BinaryOp {
        left: Box::new(Expr::Number(2)),
        right: Box::new(Expr::Number(2)),
        op: crate::ast::Operation::Add,
        return_type: Type::Any,
    };
    assert_eq!(result, expected)
}
#[test]
fn test_multi_add_binary_op() {
    let mut tokens = create_tokens(vec![
        Token::Number(2),
        Token::Add,
        Token::Number(2),
        Token::Add,
        Token::Number(4),
    ]);
    let result = parse_expression(&mut tokens).expect("parse edilemedi");
    let expected = Expr::BinaryOp {
        left: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            right: Box::new(Expr::Number(2)),
            op: crate::ast::Operation::Add,
            return_type: Type::Any,
        }),
        right: Box::new(Expr::Number(4)),
        op: crate::ast::Operation::Add,
        return_type: Type::Any,
    };
    assert_eq!(result, expected)
}
#[test]
fn test_multiply_add_binary_op() {
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
    let result = parse_expression(&mut tokens).expect("parse edilemedi");
    let result2 = parse_expression(&mut tokens2).expect("parse edilemedi");
    let expected = Expr::BinaryOp {
        left: Box::new(Expr::Number(2)),
        right: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            right: Box::new(Expr::Number(4)),
            op: crate::ast::Operation::Multiply,
            return_type: Type::Any,
        }),
        op: crate::ast::Operation::Add,
        return_type: Type::Any,
    };
    let expected2 = Expr::BinaryOp {
        left: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Number(2)),
            right: Box::new(Expr::Number(2)),
            op: crate::ast::Operation::Multiply,
            return_type: Type::Any,
        }),
        right: Box::new(Expr::Number(4)),
        op: crate::ast::Operation::Add,
        return_type: Type::Any,
    };
    assert_eq!(result, expected);
    assert_eq!(result2, expected2)
}
#[test]
fn test_equal_binary_op() {
    let mut tokens = create_tokens(vec![Token::Number(2), Token::Equal, Token::Number(2)]);
    let mut tokens2 = create_tokens(vec![Token::Number(2), Token::NotEqual, Token::Number(2)]);
    let result = parse_expression(&mut tokens).expect("parse edilemedi");
    let result2 = parse_expression(&mut tokens2).expect("parse edilemedi");
    let expected = Expr::BinaryOp {
        left: Box::new(Expr::Number(2)),
        right: Box::new(Expr::Number(2)),
        op: crate::ast::Operation::Equal,
        return_type: Type::Any,
    };
    let expected2 = Expr::BinaryOp {
        left: Box::new(Expr::Number(2)),
        right: Box::new(Expr::Number(2)),
        op: crate::ast::Operation::NotEqual,
        return_type: Type::Any,
    };
    assert_eq!(result, expected);
    assert_eq!(result2, expected2)
}
