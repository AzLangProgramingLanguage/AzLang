use parser::{
    ast::{Expr, Operation},
    shared_ast::Type,
};

use crate::runner::{Runner, runner::Value};
//TODO: Burası güncellene bilinir
pub fn binary_op_runner(
    ctx: &mut Runner,
    left: Value,
    right: Value,
    op: Operation,
    cast_type: Option<Type>,
) -> Value {
    match op {
        Operation::Add => {
            if let Some(Type::Integer) = cast_type {
                let left = left.as_number();
                let right = right.as_number();
                Value::Number(left + right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Value::Float(left + right)
            }
        }
        Operation::Subtract => {
            if let Some(Type::Integer) = cast_type {
                let left = left.as_number();
                let right = right.as_number();
                Value::Number(left - right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Value::Float(left - right)
            }
        }
        Operation::Multiply => {
            if let Some(Type::Integer) = cast_type {
                let left = left.as_number();
                let right = right.as_number();
                Value::Number(left * right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Value::Float(left * right)
            }
        }
        Operation::Divide => {
            if let Some(Type::Integer) = cast_type {
                let left = left.as_number();
                let right = right.as_number();
                Value::Number(left / right)
            } else {
                let left = left.as_float();
                let right = right.as_float();

                Value::Float(left / right)
            }
        }
        Operation::Modulo => {
            if let Some(Type::Integer) = cast_type {
                let left = left.as_number();
                let right = right.as_number();
                Value::Number(left % right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Value::Float(left % right)
            }
        }
        Operation::Equal => match (left, right) {
            (Value::Number(b), Value::Number(c)) => Value::Bool(b == c),
            (_, _) => Value::Bool(false),
        },
        Operation::NotEqual => match (left, right) {
            (Value::Number(b), Value::Number(c)) => Value::Bool(b != c),
            (_, _) => Value::Bool(false),
        },
        _ => Value::Bool(false),
    }
}
