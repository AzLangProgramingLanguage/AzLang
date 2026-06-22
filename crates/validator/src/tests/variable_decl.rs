use crate::Validator;
#[cfg(test)]
use crate::errors::ValidatorError;
use parser::{
    ast::{Atom, Expr, Symbol},
    shared_ast::{StringEnum, Type},
};
use std::{collections::HashMap, rc::Rc};

// ─── Helper ─────────────────────────────────────────────────────────────────

fn setup_validator() -> Validator {
    let mut v = Validator::default();
    v.variables.push(HashMap::new());
    v
}

// ─── Basic Declarations ────────────────────────────────────────────────────

#[test]
fn test_variable_decl_integer() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::Integer),
        false,
        Expr::Number(42),
        &mut validator,
    );
    assert!(result.is_ok());

    let sym = validator.lookup_variable("x").unwrap();
    assert_eq!(sym.typ, Type::Integer);
    assert!(!sym.is_used);
    assert!(!sym.is_mutable);
}

#[test]
fn test_variable_decl_already_declared() {
    let mut validator = setup_validator();
    validator.declare_variable(
        "x".to_string(),
        Symbol {
            typ: Type::Integer,
            is_mutable: false,
            is_used: false,
            is_changed: false,
        },
    );
    let result = crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::Integer),
        false,
        Expr::Number(2),
        &mut validator,
    );
    assert_eq!(result, Err(ValidatorError::AlreadyDecl("x".to_string())));
}

// ─── String Declarations ────────────────────────────────────────────────────

#[test]
fn test_string_decl_literal() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("s"),
        Rc::new(Type::String(StringEnum::LiteralString)),
        false,
        Expr::String(Atom::from("salam")),
        &mut validator,
    );
    assert!(result.is_ok());

    let sym = validator.lookup_variable("s").unwrap();
    assert_eq!(sym.typ, Type::String(StringEnum::LiteralString));
}

#[test]
fn test_string_decl_dynamic() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("s"),
        Rc::new(Type::String(StringEnum::DynamicString)),
        false,
        Expr::DynamicString(Rc::new("dynamic salam".to_string())),
        &mut validator,
    );
    assert!(result.is_ok());

    let sym = validator.lookup_variable("s").unwrap();
    assert_eq!(sym.typ, Type::String(StringEnum::DynamicString));
}

#[test]
fn test_string_decl_type_mismatch_int() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("s"),
        Rc::new(Type::Integer),
        false,
        Expr::String(Atom::from("salam")),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "s".to_string(),
            expected: Type::Integer.to_string(),
            found: Type::String(StringEnum::LiteralString).to_string(),
        })
    );
}

#[test]
fn test_variable_decl_already_declared_string() {
    let mut validator = setup_validator();
    validator.declare_variable(
        "s".to_string(),
        Symbol {
            typ: Type::String(StringEnum::LiteralString),
            is_mutable: false,
            is_used: false,
            is_changed: false,
        },
    );
    let result = crate::decl::validate_decl(
        Atom::from("s"),
        Rc::new(Type::String(StringEnum::LiteralString)),
        false,
        Expr::String(Atom::from("bir")),
        &mut validator,
    );
    assert_eq!(result, Err(ValidatorError::AlreadyDecl("s".to_string())));
}

// ─── Various Type Declarations ──────────────────────────────────────────────

#[test]
fn test_decl_bool() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("b"),
        Rc::new(Type::Bool),
        false,
        Expr::Bool(true),
        &mut validator,
    );
    assert!(result.is_ok());
}

#[test]
fn test_decl_float() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("f"),
        Rc::new(Type::Float),
        false,
        Expr::Float(3.15),
        &mut validator,
    );
    assert!(result.is_ok());
}

#[test]
fn test_decl_char() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("c"),
        Rc::new(Type::Char),
        false,
        Expr::Char('A'),
        &mut validator,
    );
    assert!(result.is_ok());
}

#[test]
fn test_decl_natural() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("n"),
        Rc::new(Type::Natural),
        false,
        Expr::Number(5),
        &mut validator,
    );
    assert!(result.is_ok());
}

#[test]
fn test_decl_any() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("a"),
        Rc::new(Type::Any),
        false,
        Expr::Number(42),
        &mut validator,
    );
    assert!(result.is_ok());

    let sym = validator.lookup_variable("a").unwrap();
    assert_eq!(sym.typ, Type::Integer);
}

#[test]
fn test_decl_any_with_string() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("a"),
        Rc::new(Type::Any),
        false,
        Expr::String(Atom::from("any string")),
        &mut validator,
    );
    assert!(result.is_ok());

    let sym = validator.lookup_variable("a").unwrap();
    assert_eq!(sym.typ, Type::String(StringEnum::LiteralString));
}

#[test]
fn test_decl_big_integer() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("bi"),
        Rc::new(Type::BigInteger),
        false,
        Expr::Number(999),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "bi".to_string(),
            expected: Type::BigInteger.to_string(),
            found: Type::Integer.to_string(),
        })
    );
}

#[test]
fn test_decl_low_integer() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("li"),
        Rc::new(Type::LowInteger),
        false,
        Expr::Number(1),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "li".to_string(),
            expected: Type::LowInteger.to_string(),
            found: Type::Integer.to_string(),
        })
    );
}
//
// ─── Type Mismatch Errors ───────────────────────────────────────────────────

#[test]
fn test_decl_type_mismatch_int_bool() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::Integer),
        false,
        Expr::Bool(true),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "x".to_string(),
            expected: Type::Integer.to_string(),
            found: Type::Bool.to_string(),
        })
    );
}

#[test]
fn test_decl_type_mismatch_bool_int() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("b"),
        Rc::new(Type::Bool),
        false,
        Expr::Number(1),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "b".to_string(),
            expected: Type::Bool.to_string(),
            found: Type::Integer.to_string(),
        })
    );
}

#[test]
fn test_decl_type_mismatch_int_float() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::Integer),
        false,
        Expr::Float(1.5),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "x".to_string(),
            expected: Type::Integer.to_string(),
            found: Type::Float.to_string(),
        })
    );
}

#[test]
fn test_decl_type_mismatch_bool_string() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("b"),
        Rc::new(Type::Bool),
        false,
        Expr::String(Atom::from("salam")),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "b".to_string(),
            expected: Type::Bool.to_string(),
            found: Type::String(StringEnum::LiteralString).to_string(),
        })
    );
}

#[test]
fn test_decl_type_mismatch_float_bool() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("f"),
        Rc::new(Type::Float),
        false,
        Expr::Bool(false),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "f".to_string(),
            expected: Type::Float.to_string(),
            found: Type::Bool.to_string(),
        })
    );
}

#[test]
fn test_decl_type_mismatch_char_int() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("c"),
        Rc::new(Type::Char),
        false,
        Expr::Number(65),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "c".to_string(),
            expected: Type::Char.to_string(),
            found: Type::Integer.to_string(),
        })
    );
}

// ─── Array Type Declarations ────────────────────────────────────────────────

#[test]
fn test_array_decl_integer() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("arr"),
        Rc::new(Type::Array(Box::new(Type::Integer))),
        false,
        Expr::List(vec![Expr::Number(1), Expr::Number(2), Expr::Number(3)]),
        &mut validator,
    );
    assert!(result.is_ok());

    let sym = validator.lookup_variable("arr").unwrap();
    assert_eq!(sym.typ, Type::Array(Box::new(Type::Integer)));
}

#[test]
fn test_array_decl_empty() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("arr"),
        Rc::new(Type::Array(Box::new(Type::Any))),
        false,
        Expr::List(vec![]),
        &mut validator,
    );
    assert!(result.is_ok());

    let sym = validator.lookup_variable("arr").unwrap();
    assert_eq!(sym.typ, Type::Array(Box::new(Type::Any)));
}

#[test]
fn test_array_decl_string() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("arr"),
        Rc::new(Type::Array(Box::new(Type::String(
            StringEnum::LiteralString,
        )))),
        false,
        Expr::List(vec![
            Expr::String(Atom::from("a")),
            Expr::String(Atom::from("b")),
        ]),
        &mut validator,
    );
    assert!(result.is_ok());
}

#[test]
fn test_array_decl_bool() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("arr"),
        Rc::new(Type::Array(Box::new(Type::Bool))),
        false,
        Expr::List(vec![Expr::Bool(true), Expr::Bool(false)]),
        &mut validator,
    );
    assert!(result.is_ok());
}

#[test]
fn test_array_decl_float() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("arr"),
        Rc::new(Type::Array(Box::new(Type::Float))),
        false,
        Expr::List(vec![Expr::Float(1.1), Expr::Float(2.2)]),
        &mut validator,
    );
    assert!(result.is_ok());
}

#[test]
fn test_array_decl_mixed_type_items() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("arr"),
        Rc::new(Type::Array(Box::new(Type::Integer))),
        false,
        Expr::List(vec![Expr::Number(1), Expr::String(Atom::from("x"))]),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::TypeMismatch {
            expected: Type::Integer,
            found: Type::String(StringEnum::LiteralString),
        })
    );
}

#[test]
fn test_array_decl_type_mismatch() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("arr"),
        Rc::new(Type::Integer),
        false,
        Expr::List(vec![Expr::Number(1)]),
        &mut validator,
    );
    assert_eq!(
        result,
        Err(ValidatorError::DeclTypeMismatch {
            name: "arr".to_string(),
            expected: Type::Integer.to_string(),
            found: Type::Array(Box::new(Type::Integer)).to_string(),
        })
    );
}
