use parser::{ast::Operation, shared_ast::Type};

use crate::runner::{Runner, runner::Value};
//TODO: Burası güncellene bilinir
pub fn binary_op_runner(
    _ctx: &mut Runner,
    left: Value,
    right: Value,
    op: Operation,
    cast_type: Option<Type>,
) -> Value {
    match op {
        Operation::Not => Value::Bool(false),
        Operation::Add => {
            if let Some(Type::Integer) = cast_type {
                let left = left.as_number();
                let right = right.as_number();
                Value::Number(left + right)
            } else if matches!(cast_type, Some(Type::String(_))) {
                Value::String(format!("{}{}", left, right))
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
        Operation::Equal => Value::Bool(left == right),
        Operation::NotEqual => Value::Bool(left != right),
        Operation::Less => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a < b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a < b),
            (Value::String(a), Value::String(b)) => Value::Bool(a < b),
            _ => Value::Bool(false),
        },
        Operation::LessEqual => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a <= b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a <= b),
            (Value::String(a), Value::String(b)) => Value::Bool(a <= b),
            _ => Value::Bool(false),
        },
        Operation::Greater => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a > b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a > b),
            (Value::String(a), Value::String(b)) => Value::Bool(a > b),
            _ => Value::Bool(false),
        },
        Operation::GreaterEqual => match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Bool(a >= b),
            (Value::Float(a), Value::Float(b)) => Value::Bool(a >= b),
            (Value::String(a), Value::String(b)) => Value::Bool(a >= b),
            _ => Value::Bool(false),
        },
        Operation::And => match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
            _ => Value::Bool(false),
        },
        Operation::Or => match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
            _ => Value::Bool(false),
        },
    }
}
