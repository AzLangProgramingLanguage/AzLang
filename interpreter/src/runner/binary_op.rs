use std::cmp::Ordering;

use parser::{ast::Operation, shared_ast::Type};

use crate::runner::{Runner, runner::Value};

fn value_ordering(left: &Value, right: &Value) -> Option<Ordering> {
    match (left, right) {
        (Value::Number(left), Value::Number(right)) => left.partial_cmp(right),
        (Value::Number(left), Value::Float(right)) => (*left as f64).partial_cmp(right),
        (Value::Float(left), Value::Number(right)) => left.partial_cmp(&(*right as f64)),
        (Value::Float(left), Value::Float(right)) => left.partial_cmp(right),
        (Value::String(left), Value::String(right)) => Some(left.cmp(right)),
        (Value::Bool(left), Value::Bool(right)) => Some(left.cmp(right)),
        (Value::Char(left), Value::Char(right)) => Some(left.cmp(right)),
        (Value::List(left), Value::List(right)) => {
            for (left, right) in left.iter().zip(right) {
                let ordering = value_ordering(left, right)?;
                if ordering != Ordering::Equal {
                    return Some(ordering);
                }
            }
            Some(left.len().cmp(&right.len()))
        }
        (Value::Void, Value::Void) => Some(Ordering::Equal),
        _ => None,
    }
}

fn comparison_result(left: Value, right: Value, op: Operation) -> Value {
    let ordering = value_ordering(&left, &right)
        .unwrap_or_else(|| panic!("Invalid operands for {op:?}: {left:?} and {right:?}"));
    let result = match op {
        Operation::Less => ordering == Ordering::Less,
        Operation::LessEqual => ordering != Ordering::Greater,
        Operation::Greater => ordering == Ordering::Greater,
        Operation::GreaterEqual => ordering != Ordering::Less,
        _ => unreachable!(),
    };
    Value::Bool(result)
}

pub fn binary_op_runner(
    _ctx: &mut Runner,
    left: Value,
    right: Value,
    op: Operation,
    cast_type: Option<Type>,
) -> Value {
    match op {
        Operation::Not => match right {
            Value::Bool(value) => Value::Bool(!value),
            other => panic!("Invalid operand for Not: {other:?}"),
        },
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
        Operation::Equal => Value::Bool(value_ordering(&left, &right) == Some(Ordering::Equal)),
        Operation::NotEqual => Value::Bool(value_ordering(&left, &right) != Some(Ordering::Equal)),
        Operation::Less | Operation::LessEqual | Operation::Greater | Operation::GreaterEqual => {
            comparison_result(left, right, op)
        }
        Operation::And => match (&left, &right) {
            (Value::Bool(left), Value::Bool(right)) => Value::Bool(*left && *right),
            _ => panic!("Invalid operands for And: {left:?} and {right:?}"),
        },
        Operation::Or => match (&left, &right) {
            (Value::Bool(left), Value::Bool(right)) => Value::Bool(*left || *right),
            _ => panic!("Invalid operands for Or: {left:?} and {right:?}"),
        },
    }
}
