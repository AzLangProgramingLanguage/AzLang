use crate::{Validator, ast::Ast, ast::Expr as ValidatorExpr, errors::ValidatorError};
use parser::{
    ast::{Expr, Parameter, Statement},
    shared_ast::Type,
};
use std::assert_matches;

fn make_func(
    name: &str,
    return_typ: Type,
    params: Vec<Parameter>,
    body: Vec<Statement>,
) -> Statement {
    Statement::FunctionDef {
        name: name.to_string(),
        return_typ,
        params,
        body,
    }
}

#[test]
fn test_function_call_success() {
    let stmts = vec![
        make_func("foo", Type::Integer, vec![], vec![]),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "foo".to_string(),
                symbol: None,
            }),
            args: vec![],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert!(result.is_ok());
}

#[test]
fn test_function_call_with_args() {
    let stmts = vec![
        make_func(
            "add",
            Type::Integer,
            vec![
                Parameter {
                    name: "a".to_string(),
                    typ: Type::Integer,
                    is_pointer: false,
                },
                Parameter {
                    name: "b".to_string(),
                    typ: Type::Integer,
                    is_pointer: false,
                },
            ],
            vec![],
        ),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "add".to_string(),
                symbol: None,
            }),
            args: vec![Expr::Number(1), Expr::Number(2)],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert!(result.is_ok());
}

#[test]
fn test_function_call_not_found() {
    let stmts = vec![Statement::Expr(Expr::Call {
        target: None,
        name: Box::new(Expr::VariableRef {
            name: "nonexistent".to_string(),
            symbol: None,
        }),
        args: vec![],
    })];
    let result = Validator::default().validate(stmts);
    assert_matches!(result, Err(ValidatorError::FunctionNotFound(_)));
}

#[test]
fn test_function_call_invalid_name() {
    let stmts = vec![Statement::Expr(Expr::Call {
        target: None,
        name: Box::new(Expr::Number(42)),
        args: vec![],
    })];
    let result = Validator::default().validate(stmts);
    assert_matches!(result, Err(ValidatorError::InvalidFunctionCall(_)));
}

#[test]
fn test_function_call_return_type_in_result() {
    let stmts = vec![
        make_func("foo", Type::Integer, vec![], vec![]),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "foo".to_string(),
                symbol: None,
            }),
            args: vec![],
        }),
    ];
    let (_validator, program) = Validator::default().validate(stmts).expect("should validate");
    let returned_type = match &program.expressions[0] {
        Ast::Expr(ValidatorExpr::Call { returned_type, .. }) => returned_type,
        other => panic!("expected Call, got {other:?}"),
    };
    assert_eq!(*returned_type, Type::Integer);
}

#[test]
fn test_function_call_return_type_bool() {
    let stmts = vec![
        make_func("is_valid", Type::Bool, vec![], vec![]),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: "is_valid".to_string(),
                symbol: None,
            }),
            args: vec![],
        }),
    ];
    let (_validator, program) = Validator::default().validate(stmts).expect("should validate");
    let returned_type = match &program.expressions[0] {
        Ast::Expr(ValidatorExpr::Call { returned_type, .. }) => returned_type,
        other => panic!("expected Call, got {other:?}"),
    };
    assert_eq!(*returned_type, Type::Bool);
}
