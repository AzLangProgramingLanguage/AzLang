use std::{collections::HashMap, rc::Rc};

use parser::{
    ast::{Expr, Program, Statement},
    shared_ast::Type,
};

use crate::{Validator, errors::ValidatorError};

#[test]
fn test_assignment_success() {
    let mut validator = Validator::new();
    let mut program = Program {
        expressions: vec![
            // Əvvəlcə mutable dəyişən yaradırıq
            Statement::Decl {
                name: "x".to_string(),
                typ: Rc::new(Type::Integer),
                is_mutable: true,
                value: Box::new(Expr::Number(1)),
            },
            // Sonra ona yeni dəyər veririk
            Statement::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::Number(2)),
            },
        ],
        functions: HashMap::new(),
    };
    validator.validate(&mut program).unwrap();

    let sym = validator.global_variables.get("x").expect("'x' tapılmadı");
    assert!(sym.is_changed);
    assert!(sym.is_used);
}

#[test]
fn test_assignment_to_immutable() {
    let mut validator = Validator::new();
    let mut program = Program {
        expressions: vec![
            Statement::Decl {
                name: "x".to_string(),
                typ: Rc::new(Type::Integer),
                is_mutable: false,
                value: Box::new(Expr::Number(1)),
            },
            Statement::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::Number(2)),
            },
        ],
        functions: HashMap::new(),
    };
    let result = validator.validate(&mut program);
    assert!(matches!(
        result,
        Err(ValidatorError::AssignmentToImmutableVariable(_))
    ));
}

#[test]
fn test_assignment_type_mismatch() {
    let mut validator = Validator::new();
    let mut program = Program {
        expressions: vec![
            Statement::Decl {
                name: "x".to_string(),
                typ: Rc::new(Type::Integer),
                is_mutable: true,
                value: Box::new(Expr::Number(1)),
            },
            // Integer dəyişənə String vermək cəhdi
            Statement::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::String("salam".to_string())),
            },
        ],
        functions: HashMap::new(),
    };
    let result = validator.validate(&mut program);
    assert!(matches!(
        result,
        Err(ValidatorError::AssignmentTypeMismatch { .. })
    ));
}

#[test]
fn test_assignment_undefined_variable() {
    let mut validator = Validator::new();
    let mut program = Program {
        expressions: vec![
            // Heç yaradılmamış dəyişənə assign etmək cəhdi
            Statement::Assignment {
                name: "x".to_string(),
                value: Box::new(Expr::Number(1)),
            },
        ],
        functions: HashMap::new(),
    };
    let result = validator.validate(&mut program);
    assert!(matches!(result, Err(ValidatorError::UndefinedVariable(_))));
}
