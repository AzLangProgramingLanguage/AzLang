use crate::{Validator, ast::Ast, ast::Expr as ValidatorExpr, errors::ValidatorError};
use parser::{
    ast::{Atom, Expr, Parameter, Statement},
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
        name: Atom::from(name),
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
                name: Atom::from("foo"),
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
                    name: Atom::from("a"),
                    typ: Type::Integer,
                    is_pointer: false,
                },
                Parameter {
                    name: Atom::from("b"),
                    typ: Type::Integer,
                    is_pointer: false,
                },
            ],
            vec![],
        ),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("add"),
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
            name: Atom::from("nonexistent"),
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
                name: Atom::from("foo"),
                symbol: None,
            }),
            args: vec![],
        }),
    ];
    let (_validator, program) = Validator::default()
        .validate(stmts)
        .expect("should validate");
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
                name: Atom::from("is_valid"),
                symbol: None,
            }),
            args: vec![],
        }),
    ];
    let (_validator, program) = Validator::default()
        .validate(stmts)
        .expect("should validate");
    let returned_type = match &program.expressions[0] {
        Ast::Expr(ValidatorExpr::Call { returned_type, .. }) => returned_type,
        other => panic!("expected Call, got {other:?}"),
    };
    assert_eq!(*returned_type, Type::Bool);
}

#[test]
fn test_function_call_wrong_arg_count_too_few() {
    let stmts = vec![
        make_func(
            "add",
            Type::Integer,
            vec![
                Parameter {
                    name: Atom::from("a"),
                    typ: Type::Integer,
                    is_pointer: false,
                },
                Parameter {
                    name: Atom::from("b"),
                    typ: Type::Integer,
                    is_pointer: false,
                },
            ],
            vec![],
        ),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("add"),
                symbol: None,
            }),
            args: vec![Expr::Number(1)],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert_matches!(
        result,
        Err(ValidatorError::InvalidArgumentCount { .. })
    );
}

#[test]
fn test_function_call_wrong_arg_count_too_many() {
    let stmts = vec![
        make_func(
            "foo",
            Type::Void,
            vec![Parameter {
                name: Atom::from("a"),
                typ: Type::Integer,
                is_pointer: false,
            }],
            vec![],
        ),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("foo"),
                symbol: None,
            }),
            args: vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert_matches!(
        result,
        Err(ValidatorError::InvalidArgumentCount { .. })
    );
}

#[test]
fn test_function_call_wrong_arg_type() {
    let stmts = vec![
        make_func(
            "foo",
            Type::Void,
            vec![Parameter {
                name: Atom::from("a"),
                typ: Type::Integer,
                is_pointer: false,
            }],
            vec![],
        ),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("foo"),
                symbol: None,
            }),
            args: vec![Expr::String(Atom::from("hello"))],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert_matches!(
        result,
        Err(ValidatorError::InvalidArgumentType { .. })
    );
}

#[test]
fn test_function_call_multiple_args_type_check() {
    let stmts = vec![
        make_func(
            "add",
            Type::Integer,
            vec![
                Parameter {
                    name: Atom::from("a"),
                    typ: Type::Integer,
                    is_pointer: false,
                },
                Parameter {
                    name: Atom::from("b"),
                    typ: Type::Integer,
                    is_pointer: false,
                },
            ],
            vec![],
        ),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("add"),
                symbol: None,
            }),
            args: vec![Expr::Number(1), Expr::String(Atom::from("wrong"))],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert_matches!(
        result,
        Err(ValidatorError::InvalidArgumentType { .. })
    );
}

fn make_external_func(
    name: &str,
    return_typ: Type,
    params: Vec<Parameter>,
    library: &str,
    symbol: &str,
) -> Statement {
    Statement::ExternalFunctionDef {
        name: Atom::from(name),
        return_typ,
        params,
        library: Atom::from(library),
        symbol: Atom::from(symbol),
        link_name: None,
    }
}

#[test]
fn test_external_function_call_success() {
    let stmts = vec![
        make_external_func("add", Type::Integer, vec![Parameter {
            name: Atom::from("a"),
            typ: Type::Integer,
            is_pointer: false,
        }], "c", "add"),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("add"),
                symbol: None,
            }),
            args: vec![Expr::Number(42)],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert!(result.is_ok());
}

#[test]
fn test_external_function_call_wrong_arg_count() {
    let stmts = vec![
        make_external_func("foo", Type::Void, vec![
            Parameter {
                name: Atom::from("a"),
                typ: Type::Integer,
                is_pointer: false,
            },
            Parameter {
                name: Atom::from("b"),
                typ: Type::Integer,
                is_pointer: false,
            },
        ], "c", "foo"),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("foo"),
                symbol: None,
            }),
            args: vec![Expr::Number(1)],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert_matches!(
        result,
        Err(ValidatorError::InvalidArgumentCount { .. })
    );
}

#[test]
fn test_external_function_call_wrong_arg_type() {
    let stmts = vec![
        make_external_func("print_int", Type::Void, vec![Parameter {
            name: Atom::from("x"),
            typ: Type::Integer,
            is_pointer: false,
        }], "c", "print_int"),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("print_int"),
                symbol: None,
            }),
            args: vec![Expr::String(Atom::from("hello"))],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert_matches!(
        result,
        Err(ValidatorError::InvalidArgumentType { .. })
    );
}

#[test]
fn test_external_function_call_no_args() {
    let stmts = vec![
        make_external_func("get_time", Type::Integer, vec![], "c", "time"),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("get_time"),
                symbol: None,
            }),
            args: vec![],
        }),
    ];
    let result = Validator::default().validate(stmts);
    assert!(result.is_ok());
}

#[test]
fn test_external_func_link_name_passed_through() {
    let stmts = vec![
        Statement::ExternalFunctionDef {
            name: Atom::from("print"),
            return_typ: Type::Void,
            params: vec![Parameter {
                name: Atom::from("val"),
                typ: Type::Any,
                is_pointer: false,
            }],
            library: Atom::from("../build/printlib.so"),
            symbol: Atom::from("printValue"),
            link_name: Some(Atom::from("printlib")),
        },
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("print"),
                symbol: None,
            }),
            args: vec![Expr::Number(42)],
        }),
    ];
    let (_validator, program) = Validator::default()
        .validate(stmts)
        .expect("should validate with link_name");
    assert_eq!(program.external_functions.len(), 1);
    assert_eq!(
        program.external_functions[0].link_name,
        Some("printlib".to_string())
    );
}

#[test]
fn test_external_func_link_name_none_when_omitted() {
    let stmts = vec![
        make_external_func("print", Type::Void, vec![Parameter {
            name: Atom::from("val"),
            typ: Type::Any,
            is_pointer: false,
        }], "../build/printlib.so", "printValue"),
        Statement::Expr(Expr::Call {
            target: None,
            name: Box::new(Expr::VariableRef {
                name: Atom::from("print"),
                symbol: None,
            }),
            args: vec![Expr::Number(42)],
        }),
    ];
    let (_validator, program) = Validator::default()
        .validate(stmts)
        .expect("should validate without link_name");
    assert_eq!(program.external_functions.len(), 1);
    assert!(program.external_functions[0].link_name.is_none());
}
