use parser::{ast::{Expr, Operation}, shared_ast::Type};

use crate::runner::{Runner, runner::runner_interpretator};

pub fn binary_op_runner<'a>(
    ctx: &mut Runner<'a>,
    left: Box<Expr<'a>>,
    right: Box<Expr<'a>>,
    op: Operation,
    return_type: Type<'a>,
) -> Expr<'a> {
    let left = runner_interpretator(ctx, *left);
    let right = runner_interpretator(ctx, *right);

    let result = match op {
        Operation::Add => {
            if let Type::Integer = return_type {
                let left = left.as_number();
                let right = right.as_number();
                Expr::Number(left + right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Expr::Float(left + right)
            }
        }
        Operation::Subtract => {
            if let Type::Integer = return_type {
                let left = left.as_number();
                let right = right.as_number();
                Expr::Number(left - right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Expr::Float(left - right)
            }
        }
        Operation::Multiply => {
            if let Type::Integer = return_type {
                let left = left.as_number();
                let right = right.as_number();
                Expr::Number(left * right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Expr::Float(left * right)
            }
        }
        Operation::Divide => {
            if let Type::Integer = return_type {

                let left = left.as_number();
                let right = right.as_number();
                Expr::Number(left / right)
            } else {
                let left = left.as_float();
                let right = right.as_float();

                Expr::Float(left / right)
            }
        }
        Operation::Modulo => {
            if let Type::Natural = return_type {
                let left = left.as_number();
                let right = right.as_number();
                Expr::Number(left % right)
            } else {
                let left = left.as_float();
                let right = right.as_float();
                Expr::Float(left % right)
            }
        }
        Operation::Equal => {
            match (left, right) {
                (Expr::Number(b), Expr::Number(c)) => Expr::Bool(b == c),
                (_, _) => Expr::Bool(false),
            }
           
        }
        Operation::NotEqual => {
                Expr::Bool(false)  
        }
        _ => Expr::Bool(false),
    };

    result
}
