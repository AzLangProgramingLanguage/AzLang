use crate::Validator;
use parser::{
    ast::{Atom, Expr, Parameter, Statement},
    shared_ast::Type,
};
use std::rc::Rc;

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

fn make_decl(name: &str, typ: Type, is_mutable: bool, value: Expr) -> Statement {
    Statement::Decl {
        name: Atom::from(name),
        typ: Rc::new(typ),
        is_mutable,
        value: Box::new(value),
    }
}

#[test]
fn test_function_decl_registers_function() {
    let mut validator = Validator::default();
    let stmt = make_func("foo", Type::Integer, vec![], vec![]);

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("foo").expect("foo should be registered");
    assert_eq!(info.return_type, Type::Integer);
    assert!(info.parameters.is_empty());
}

#[test]
fn test_function_decl_return_type_void() {
    let mut validator = Validator::default();
    let stmt = make_func("bar", Type::Void, vec![], vec![]);

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("bar").expect("bar should be registered");
    assert_eq!(info.return_type, Type::Void);
}

#[test]
fn test_function_decl_return_type_string() {
    let mut validator = Validator::default();
    let stmt = make_func(
        "greet",
        Type::String(parser::shared_ast::StringEnum::DynamicString),
        vec![],
        vec![],
    );

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("greet").expect("greet should be registered");
    assert_eq!(
        info.return_type,
        Type::String(parser::shared_ast::StringEnum::DynamicString)
    );
}

#[test]
fn test_function_decl_return_type_bool() {
    let mut validator = Validator::default();
    let stmt = make_func("is_valid", Type::Bool, vec![], vec![]);

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("is_valid").expect("is_valid should be registered");
    assert_eq!(info.return_type, Type::Bool);
}

#[test]
fn test_function_decl_return_type_float() {
    let mut validator = Validator::default();
    let stmt = make_func("calc", Type::Float, vec![], vec![]);

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("calc").expect("calc should be registered");
    assert_eq!(info.return_type, Type::Float);
}

#[test]
fn test_function_decl_parameters_single() {
    let mut validator = Validator::default();
    let params = vec![Parameter {
        name: Atom::from("x"),
        typ: Type::Integer,
        is_pointer: false,
    }];
    let stmt = make_func("f", Type::Integer, params, vec![]);

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("f").expect("f should be registered");
    assert_eq!(info.parameters.len(), 1);
    assert_eq!(info.parameters[0].name, Atom::from("x"));
    assert_eq!(info.parameters[0].typ, Type::Integer);
}

#[test]
fn test_function_decl_parameters_multiple() {
    let mut validator = Validator::default();
    let params = vec![
        Parameter {
            name: Atom::from("a"),
            typ: Type::Integer,
            is_pointer: false,
        },
        Parameter {
            name: Atom::from("b"),
            typ: Type::String(parser::shared_ast::StringEnum::DynamicString),
            is_pointer: false,
        },
        Parameter {
            name: Atom::from("c"),
            typ: Type::Bool,
            is_pointer: false,
        },
    ];
    let stmt = make_func("multi", Type::Void, params, vec![]);

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("multi").expect("multi should be registered");
    assert_eq!(info.parameters.len(), 3);
    assert_eq!(info.parameters[0].name, Atom::from("a"));
    assert_eq!(info.parameters[0].typ, Type::Integer);
    assert_eq!(info.parameters[1].name, Atom::from("b"));
    assert_eq!(info.parameters[1].typ, Type::String(parser::shared_ast::StringEnum::DynamicString));
    assert_eq!(info.parameters[2].name, Atom::from("c"));
    assert_eq!(info.parameters[2].typ, Type::Bool);
}

#[test]
fn test_function_decl_parameter_pointer_flag() {
    let mut validator = Validator::default();
    let params = vec![Parameter {
        name: Atom::from("ptr"),
        typ: Type::Integer,
        is_pointer: true,
    }];
    let stmt = make_func("deref", Type::Integer, params, vec![]);

    validator.function_decl(&vec![stmt]);

    let info = validator.functions.get("deref").expect("deref should be registered");
    assert_eq!(info.parameters.len(), 1);
    assert!(info.parameters[0].is_pointer);
}

#[test]
fn test_function_decl_multiple_functions() {
    let mut validator = Validator::default();
    let stmts = vec![
        make_func("a", Type::Integer, vec![], vec![]),
        make_func("b", Type::Bool, vec![], vec![]),
        make_func("c", Type::Void, vec![], vec![]),
    ];

    validator.function_decl(&stmts);

    assert!(validator.functions.contains_key("a"));
    assert!(validator.functions.contains_key("b"));
    assert!(validator.functions.contains_key("c"));
    assert_eq!(validator.functions.len(), 3);
    assert_eq!(
        validator.functions.get("a").unwrap().return_type,
        Type::Integer
    );
    assert_eq!(
        validator.functions.get("b").unwrap().return_type,
        Type::Bool
    );
    assert_eq!(
        validator.functions.get("c").unwrap().return_type,
        Type::Void
    );
}

#[test]
fn test_function_decl_ignores_decl_statements() {
    let mut validator = Validator::default();
    let stmts = vec![
        make_func("f", Type::Integer, vec![], vec![]),
        make_decl("x", Type::Integer, false, Expr::Number(1)),
    ];

    validator.function_decl(&stmts);

    assert!(validator.functions.contains_key("f"));
    assert_eq!(validator.functions.len(), 1);
}

#[test]
fn test_function_decl_overwrites_duplicate() {
    let mut validator = Validator::default();
    let stmts = vec![
        make_func("dup", Type::Integer, vec![], vec![]),
        make_func("dup", Type::Bool, vec![], vec![]),
    ];

    validator.function_decl(&stmts);

    let info = validator.functions.get("dup").expect("dup should be registered");
    assert_eq!(info.return_type, Type::Bool);
}

#[test]
fn test_function_decl_returns_self() {
    let mut validator = Validator::default();
    let stmt = make_func("f", Type::Integer, vec![], vec![]);
    let result = validator.function_decl(&vec![stmt]);
    assert_eq!(result as *mut Validator, &mut validator as *mut Validator);
}

#[test]
fn test_function_decl_empty_input() {
    let mut validator = Validator::default();
    validator.function_decl(&vec![]);
    assert!(validator.functions.is_empty());
}

#[test]
fn test_validate_with_function_def_no_error() {
    let stmts = vec![make_func("f", Type::Integer, vec![], vec![])];
    let result = Validator::default().validate(stmts);
    assert!(result.is_ok());
}

#[test]
fn test_validate_with_function_def_and_return_expr() {
    let stmts = vec![make_func(
        "f",
        Type::Integer,
        vec![],
        vec![Statement::Expr(Expr::Return(Box::new(Expr::Number(42))))],
    )];
    let result = Validator::default().validate(stmts);
    assert!(result.is_ok());
}

#[test]
fn test_validate_with_multiple_function_defs() {
    let stmts = vec![
        make_func("a", Type::Integer, vec![], vec![]),
        make_func("b", Type::Void, vec![], vec![]),
    ];
    let result = Validator::default().validate(stmts);
    assert!(result.is_ok());
}

#[test]
fn test_validate_function_def_preserves_functions_in_result() {
    let stmts = vec![
        make_func("foo", Type::Integer, vec![], vec![]),
        make_func("bar", Type::Bool, vec![], vec![]),
    ];
    let (validator, _program) = Validator::default().validate(stmts).expect("should validate");

    assert!(validator.functions.contains_key("foo"));
    assert!(validator.functions.contains_key("bar"));
    assert_eq!(validator.functions.len(), 2);
    assert_eq!(
        validator.functions.get("foo").unwrap().return_type,
        Type::Integer
    );
    assert_eq!(
        validator.functions.get("bar").unwrap().return_type,
        Type::Bool
    );
}

#[test]
fn test_validate_function_def_with_complex_return_type() {
    let stmts = vec![make_func(
        "get_list",
        Type::Array(Box::new(Type::Integer)),
        vec![],
        vec![],
    )];
    let (validator, _program) = Validator::default().validate(stmts).expect("should validate");

    let info = validator.functions.get("get_list").unwrap();
    assert_eq!(
        info.return_type,
        Type::Array(Box::new(Type::Integer))
    );
}
