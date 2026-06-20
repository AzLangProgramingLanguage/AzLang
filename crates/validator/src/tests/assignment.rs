use std::collections::HashMap;

use crate::{validate::validate_statement, Validator, errors::ValidatorError};
use parser::{
    ast::{Expr, Statement, Symbol},
    shared_ast::Type,
};
use std::assert_matches;
// ─── Helpers ────────────────────────────────────────────────────────────────

fn assign(name: &str, value: Expr) -> Statement {
    Statement::Assignment {
        name: name.to_string(),
        value: Box::new(value),
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[test]
fn test_assignment_success() {
    let mut validator = Validator::default();
    validator.variables.push(HashMap::new());
    validator.declare_variable(
        "x".to_string(),
        Symbol {
            typ: Type::Integer,
            is_mutable: true,
            is_used: false,
            is_changed: false,
        },
    );

    let result = validate_statement(assign("x", Expr::Number(2)), &mut validator);
    assert!(result.is_ok(), "valid assignment should not fail");

    let symbol = validator
        .variables
        .last()
        .expect("variable stack should not be empty")
        .get("x")
        .expect("symbol 'x' should exist in scope");

    assert!(
        symbol.is_changed,
        "x should be marked as changed after assignment"
    );
}

#[test]
fn test_assignment_to_immutable() {
    let mut validator = Validator::default();
    validator.variables.push(HashMap::new());
    validator.declare_variable(
        "x".to_string(),
        Symbol {
            typ: Type::Integer,
            is_mutable: false,
            is_used: false,
            is_changed: false,
        },
    );

    let result = validate_statement(assign("x", Expr::Number(2)), &mut validator);
    assert_matches!(result, Err(ValidatorError::AssignmentToImmutableVariable(_)));
}

#[test]
fn test_assignment_type_mismatch() {
    let mut validator = Validator::default();
    validator.variables.push(HashMap::new());
    validator.declare_variable(
        "x".to_string(),
        Symbol {
            typ: Type::Integer,
            is_mutable: true,
            is_used: false,
            is_changed: false,
        },
    );

    let result = validate_statement(
        assign("x", Expr::String("salam".to_string())),
        &mut validator,
    );
    assert_matches!(result, Err(ValidatorError::AssignmentTypeMismatch { .. }));
}

#[test]
fn test_assignment_undefined_variable() {
    let mut validator = Validator::default();
    validator.variables.push(HashMap::new());

    let result = validate_statement(assign("x", Expr::Number(1)), &mut validator);
    assert_matches!(result, Err(ValidatorError::UndefinedVariable(_)));
}
