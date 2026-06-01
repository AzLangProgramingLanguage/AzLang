use std::f64::consts::PI;

use crate::{
    ast::{Expr, Operation, Statement},
    binary_op::{parse_expression, parse_statement},
    tests::create_tokens,
};
use tokenizer::tokens::Token;

#[test]
fn test_parse_function_call_no_args() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![]
        })
    )
}

#[test]
fn test_parse_function_call_single_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::Number(5),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::Number(5)]
        })
    )
}

#[test]
fn test_parse_function_call_multiple_args() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::Number(1),
        Token::Comma,
        Token::Number(2),
        Token::Comma,
        Token::Number(3),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]
        })
    )
}

#[test]
fn test_parse_function_call_string_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::StringLiteral("hello".to_string()),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::String("hello".to_string())]
        })
    )
}

#[test]
fn test_parse_function_call_bool_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::True,
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::Bool(true)]
        })
    )
}

#[test]
fn test_parse_function_call_variable_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::Identifier("y".to_string()),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::VariableRef {
                name: "y".to_string(),
                symbol: None
            }]
        })
    )
}

#[test]
fn test_parse_function_call_float_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::Float(PI),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::Float(PI)]
        })
    )
}

#[test]
fn test_parse_function_call_mixed_args() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::Number(1),
        Token::Comma,
        Token::StringLiteral("hello".to_string()),
        Token::Comma,
        Token::True,
        Token::Comma,
        Token::Identifier("z".to_string()),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![
                Expr::Number(1),
                Expr::String("hello".to_string()),
                Expr::Bool(true),
                Expr::VariableRef {
                    name: "z".to_string(),
                    symbol: None
                }
            ]
        })
    )
}

#[test]
fn test_parse_function_call_expression_direct() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("foo".to_string()),
        Token::LParen,
        Token::Number(42),
        Token::RParen,
    ]);

    let result = parse_expression(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "foo".to_string(),
                symbol: None
            }),
            args: vec![Expr::Number(42)]
        }
    )
}

#[test]
fn test_parse_function_call_neg_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::Subtract,
        Token::Number(5),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::Number(-5)]
        })
    )
}

#[test]
fn test_parse_function_call_not_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("x".to_string()),
        Token::LParen,
        Token::Not,
        Token::Identifier("flag".to_string()),
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "x".to_string(),
                symbol: None
            }),
            args: vec![Expr::UnaryOp {
                op: Operation::Not,
                expr: Box::new(Expr::VariableRef {
                    name: "flag".to_string(),
                    symbol: None
                })
            }]
        })
    )
}

#[test]
fn test_parse_function_call_two_args_expression() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("add".to_string()),
        Token::LParen,
        Token::Number(10),
        Token::Comma,
        Token::Number(20),
        Token::RParen,
    ]);

    let result = parse_expression(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "add".to_string(),
                symbol: None
            }),
            args: vec![Expr::Number(10), Expr::Number(20)]
        }
    )
}

#[test]
fn test_parse_function_call_false_arg() {
    let mut tokens = create_tokens(vec![
        Token::Identifier("f".to_string()),
        Token::LParen,
        Token::False,
        Token::RParen,
    ]);

    let result = parse_statement(&mut tokens).expect("Parse failed");

    assert_eq!(
        result,
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "f".to_string(),
                symbol: None
            }),
            args: vec![Expr::Bool(false)]
        })
    )
}
