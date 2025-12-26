use parser::{ast::Expr, shared_ast::Type};

use crate::runner::{Runner, runner::runner_interpretator};

pub fn binary_op_runner<'a>(
    ctx: &mut Runner<'a>,
    left: Box<Expr<'a>>,
    right: Box<Expr<'a>>,
    op: &'a str,
    return_type: Type<'a>,
) -> Expr<'a> {
    let left = runner_interpretator(ctx, *left);
    let right = runner_interpretator(ctx, *right);
    let result = match op {
        "+" => {
            if let Type::Natural = return_type {
                let left = left.as_number().unwrap();
                let right = right.as_number().unwrap();
                Expr::Number(left + right)
            } else {
                let left = left.as_float().unwrap();
                let right = right.as_float().unwrap();
                Expr::Float(left + right)
            }
        }
        "-" => {
            if let Type::Natural = return_type {
                let left = left.as_number().unwrap();
                let right = right.as_number().unwrap();
                Expr::Number(left - right)
            } else {
                let left = left.as_float().unwrap();
                let right = right.as_float().unwrap();
                Expr::Float(left - right)
            }
        }
        "*" => {
            if let Type::Natural = return_type {
                let left = left.as_number().unwrap();
                let right = right.as_number().unwrap();
                Expr::Number(left * right)
            } else {
                let left = left.as_float().unwrap();
                let right = right.as_float().unwrap();
                Expr::Float(left * right)
            }
        }
        "/" => {
            if let Type::Natural = return_type {
                let left = left.as_number().unwrap();
                let right = right.as_number().unwrap();
                Expr::Number(left / right)
            } else {
                let left = left.as_float().unwrap();
                let right = right.as_float().unwrap();
                Expr::Float(left / right)
            }
        }
        "%" => {
            if let Type::Natural = return_type {
                let left = left.as_number().unwrap();
                let right = right.as_number().unwrap();
                Expr::Number(left % right)
            } else {
                let left = left.as_float().unwrap();
                let right = right.as_float().unwrap();
                Expr::Float(left % right)
            }
        }
        /*  "==" => {
            if left == right {
                Expr::Bool(true)
            } else {
                Expr::Bool(false)
            }
        }
        "!=" => {
            if left != right {
                Expr::Bool(true)
            } else {
                Expr::Bool(false)
            }
        } */
        _ => Expr::Bool(false),
    };

    result
}
