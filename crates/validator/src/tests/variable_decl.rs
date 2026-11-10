use crate::Validator;
#[cfg(test)]
use crate::errors::ValidatorError;
use parser::{
    ast::{Atom, Expr, Symbol},
    shared_ast::{StringEnum, Type},
};
use std::{collections::HashMap, rc::Rc};

fn setup_validator() -> Validator {
    let mut v = Validator::default();
    v.variables.push(HashMap::new());
    v
}
#[test]
fn test_variable_decl_integer() {
    let mut validator = setup_validator();
    let result = crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::Integer),
        false,
        Box::new(Expr::Number(42)),
        &mut validator,
    );
    let mut variables: HashMap<Atom, Symbol> = HashMap::new();
    variables.insert(
        Atom::from("x"),
        Symbol {
            typ: Type::Integer,
            is_mutable: false,
            is_used: false,
            is_changed: false,
            val: Box::new(Expr::Number(42)),
        },
    );

    assert!(result.is_ok());
    assert_eq!(validator.variables.last(), Some(&variables))
}

#[test]
fn test_variable_decl_already_declared() {
    let mut validator = setup_validator();

    _ = crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::Integer),
        false,
        Box::new(Expr::Number(42)),
        &mut validator,
    );

    let result = crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::Integer),
        false,
        Box::new(Expr::Number(42)),
        &mut validator,
    );
    assert_eq!(result, Err(ValidatorError::AlreadyDecl("x".to_string())));
}

// ─── String Declarations ────────────────────────────────────────────────────

#[test]
fn test_string_decl_literal() -> Result<(), ValidatorError> {
    let mut validator = setup_validator();

    assert_eq!(
        crate::decl::validate_decl(
            Atom::from("x"),
            Rc::new(Type::Integer),
            false,
            Box::new(Expr::String(Atom::from("Hello"))),
            &mut validator,
        ),
        Err(ValidatorError::TypeMismatch {
            expected: Type::Integer,
            found: Type::String(StringEnum::LiteralString)
        })
    );
    crate::decl::validate_decl(
        Atom::from("x"),
        Rc::new(Type::String(StringEnum::LiteralString)),
        false,
        Box::new(Expr::String(Atom::from("Hello"))),
        &mut validator,
    )?;

    let sym = validator
        .lookup_variable(&Atom::from("x"))
        .expect("Variable doens't find");
    assert_eq!(sym.typ, Type::String(StringEnum::LiteralString));
    Ok(())
}
#[test]
fn test_bool_decl() -> Result<(), ValidatorError> {
    let mut validator = setup_validator();

    assert!(
        crate::decl::validate_decl(
            Atom::from("x"),
            Rc::new(Type::Bool),
            false,
            Box::new(Expr::Bool(false)),
            &mut validator,
        )
        .is_ok()
    );

    let sym = validator
        .lookup_variable(&Atom::from("x"))
        .expect("Variable doens't find");
    assert_eq!(sym.typ, Type::Bool);
    assert_ne!(sym.val, Box::new(Expr::Bool(true)));
    Ok(())
}
