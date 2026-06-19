#[cfg(test)]
use crate::{Validator, errors::ValidatorError};
use parser::{
    ast::{Expr, Statement},
    shared_ast::Type,
};
use std::assert_matches;
use std::rc::Rc;

// ─── Helpers ────────────────────────────────────────────────────────────────

fn decl(name: &str, typ: Type, is_mutable: bool, value: Expr) -> Statement {
    Statement::Decl {
        name: name.to_string(),
        typ: Rc::new(typ),
        is_mutable,
        value: Box::new(value),
    }
}

fn assign(name: &str, value: Expr) -> Statement {
    Statement::Assignment {
        name: name.to_string(),
        value: Box::new(value),
    }
}
// ─── Tests ──────────────────────────────────────────────────────────────────

#[test]
fn test_assignment_success() {
    let validator = Validator::default();
    let result = validator
        .validate(vec![
            decl("x", Type::Integer, true, Expr::Number(1)),
            assign("x", Expr::Number(2)),
        ])
        .expect("valid program should not fail");

    let symbol = result
        .0
        .variables
        .last()
        .expect("variable stack should not be empty")
        .get("x")
        .expect("symbol 'x' should exist in scope");

    assert!(
        symbol.is_changed,
        "x should be marked as changed after assignment"
    );
    assert!(
        symbol.is_used,
        "x should be marked as used after assignment"
    );
}

#[test]
fn test_assignment_to_immutable() {
    let result = Validator::default().validate(vec![
        decl("x", Type::Integer, false, Expr::Number(1)),
        assign("x", Expr::Number(2)),
    ]);

    assert_matches!(
        result,
        Err(ValidatorError::AssignmentToImmutableVariable(_))
    );
}

#[test]
fn test_assignment_type_mismatch() {
    let result = Validator::default().validate(vec![
        decl("x", Type::Integer, true, Expr::Number(1)),
        assign("x", Expr::String("salam".to_string())),
    ]);

    assert_matches!(result, Err(ValidatorError::AssignmentTypeMismatch { .. }));
}

#[test]
fn test_assignment_undefined_variable() {
    let result = Validator::default().validate(vec![assign("x", Expr::Number(1))]);

    assert_matches!(result, Err(ValidatorError::UndefinedVariable(_)));
}
