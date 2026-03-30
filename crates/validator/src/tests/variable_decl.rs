
use crate::Validator;
use parser::{
    ast::{Expr, Program, Statement},
    shared_ast::Type,
};
use std::{collections::HashMap, rc::Rc};

#[test]
fn test_variable_decl() {
    let mut validator = Validator::new();
    let mut program = Program {
        expressions: vec![Statement::Decl {
            name: "x".to_string(),
            typ: Rc::new(Type::Integer),
            is_mutable: false,
            value: Box::new(Expr::Number(1)),
        }],
        functions: HashMap::new(),
    };
    validator.validate(&mut program).unwrap();

    // Dəyişənin düzgün yaradıldığını yoxla
    let sym = validator
        .global_variables
        .get("x")
        .expect("'x' dəyişəni tapılmadı");
    assert_eq!(sym.typ, Type::Integer);
    assert_eq!(sym.is_mutable, false);
}

#[test]
fn test_variable_decl_already_declared() {
    let mut validator = Validator::new();
    let mut program = Program {
        expressions: vec![
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
        ],
        functions: HashMap::new(),
    };
    // Eyni adda iki dəyişən -> AlreadyDecl xətası gözlənilir
    let result = validator.validate(&mut program);
    assert!(result.is_err());
}

#[test]
fn test_mutable_variable_never_changed() {
    let mut validator = Validator::new();
    let mut program = Program {
        expressions: vec![Statement::Decl {
            name: "y".to_string(),
            typ: Rc::new(Type::Integer),
            is_mutable: true,
            value: Box::new(Expr::Number(5)),
        }],
        functions: HashMap::new(),
    };
    // Mutable yaradılıb amma heç vaxt dəyişdirilməyib -> NeverChangedMuttableVariable
    let result = validator.validate(&mut program);
    assert!(result.is_err());
}
