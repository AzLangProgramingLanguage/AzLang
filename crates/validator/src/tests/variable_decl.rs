#[cfg(test)]
use crate::{Validator, errors::ValidatorError};
use parser::{
    ast::{
        Expr,
        Statement::{self},
        Symbol,
    },
    shared_ast::Type,
};
use std::{collections::HashMap, rc::Rc};

#[test]
fn test_variable_decl() {
    let validator = Validator::default();
    let program = vec![Statement::Decl {
        name: "x".to_string(),
        typ: Rc::new(Type::Integer),
        is_mutable: false,
        value: Box::new(Expr::Number(1)),
    }];
    let result = validator.validate(program);
    assert_eq!(
        result,
        Err(ValidatorError::NotUsedVariable("x".to_string()))
    );
}

#[test]
fn test_variable_decl_already_declared() {
    let validator = Validator::default();
    let program = vec![
        Statement::Decl {
            name: "x".to_string(),
            typ: Rc::new(Type::Integer),
            is_mutable: false,
            value: Box::new(Expr::Number(1)),
        },
        Statement::Decl {
            name: "x".to_string(),
            typ: Rc::new(Type::Integer),
            is_mutable: false,
            value: Box::new(Expr::Number(2)),
        },
    ];
    let result = validator.validate(program);
    assert_eq!(result, Err(ValidatorError::AlreadyDecl("x".to_string())))
}


