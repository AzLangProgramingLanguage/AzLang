#[cfg(test)]
use crate::Validator;
use parser::{
    ast::{Expr, Operation, Statement},
    shared_ast::Type,
};
#[test]
fn test_binary_op_add_integers() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(2)),
        right: Box::new(Expr::Number(3)),
        op: Operation::Add,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(program.expressions.len(), 1);
    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(2)),
            right: Box::new(crate::ast::Expr::Number(3)),
            op: Operation::Add,
            return_type: Type::Integer,
        })
    );
}

#[test]
fn test_binary_op_subtract_integers() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(10)),
        right: Box::new(Expr::Number(4)),
        op: Operation::Subtract,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(10)),
            right: Box::new(crate::ast::Expr::Number(4)),
            op: Operation::Subtract,
            return_type: Type::Integer,
        })
    );
}

#[test]
fn test_binary_op_multiply_floats() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Float(2.5)),
        right: Box::new(Expr::Float(3.0)),
        op: Operation::Multiply,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Float(2.5)),
            right: Box::new(crate::ast::Expr::Float(3.0)),
            op: Operation::Multiply,
            return_type: Type::Float,
        })
    );
}

#[test]
fn test_binary_op_mixed_int_float() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(5)),
        right: Box::new(Expr::Float(2.0)),
        op: Operation::Add,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(5)),
            right: Box::new(crate::ast::Expr::Float(2.0)),
            op: Operation::Add,
            return_type: Type::Float,
        })
    );
}

#[test]
fn test_binary_op_divide_integers() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(10)),
        right: Box::new(Expr::Number(3)),
        op: Operation::Divide,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(10)),
            right: Box::new(crate::ast::Expr::Number(3)),
            op: Operation::Divide,
            return_type: Type::Integer,
        })
    );
}

#[test]
fn test_binary_op_equal_comparison() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(5)),
        right: Box::new(Expr::Number(5)),
        op: Operation::Equal,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(5)),
            right: Box::new(crate::ast::Expr::Number(5)),
            op: Operation::Equal,
            return_type: Type::Bool,
        })
    );
}

#[test]
fn test_binary_op_not_equal_comparison() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(5)),
        right: Box::new(Expr::Number(3)),
        op: Operation::NotEqual,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(5)),
            right: Box::new(crate::ast::Expr::Number(3)),
            op: Operation::NotEqual,
            return_type: Type::Bool,
        })
    );
}

#[test]
fn test_binary_op_greater_comparison() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(10)),
        right: Box::new(Expr::Number(5)),
        op: Operation::Greater,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(10)),
            right: Box::new(crate::ast::Expr::Number(5)),
            op: Operation::Greater,
            return_type: Type::Bool,
        })
    );
}

#[test]
fn test_binary_op_less_equal_comparison() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(3)),
        right: Box::new(Expr::Number(3)),
        op: Operation::LessEqual,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(3)),
            right: Box::new(crate::ast::Expr::Number(3)),
            op: Operation::LessEqual,
            return_type: Type::Bool,
        })
    );
}

#[test]
fn test_binary_op_logical_and() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Bool(true)),
        right: Box::new(Expr::Bool(false)),
        op: Operation::And,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Bool(true)),
            right: Box::new(crate::ast::Expr::Bool(false)),
            op: Operation::And,
            return_type: Type::Bool,
        })
    );
}

#[test]
fn test_binary_op_logical_or() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Bool(true)),
        right: Box::new(Expr::Bool(false)),
        op: Operation::Or,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Bool(true)),
            right: Box::new(crate::ast::Expr::Bool(false)),
            op: Operation::Or,
            return_type: Type::Bool,
        })
    );
}

#[test]
fn test_binary_op_modulo() {
    let stmt = Statement::Expr(Expr::BinaryOp {
        left: Box::new(Expr::Number(10)),
        right: Box::new(Expr::Number(3)),
        op: Operation::Modulo,
    });
    let (_validator, program) = Validator::default()
        .validate(vec![stmt])
        .expect("valid program should not fail");

    assert_eq!(
        program.expressions[0],
        crate::ast::Ast::Expr(crate::ast::Expr::BinaryOp {
            left: Box::new(crate::ast::Expr::Number(10)),
            right: Box::new(crate::ast::Expr::Number(3)),
            op: Operation::Modulo,
            return_type: Type::Integer,
        })
    );
}
