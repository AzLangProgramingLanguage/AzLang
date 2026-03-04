use parser::{
    ast::{Expr, Operation},
    shared_ast::Type,
};

use crate::{TranspileContext, transpile::transpile_expr};

pub fn transpile_binary_op(
    ctx: &mut TranspileContext,
    left: Box<Expr>,
    right: Box<Expr>,
    op: Operation,
    return_type: Type,
) -> String {
    let left = transpile_expr(*left, ctx);
    let right = transpile_expr(*right, ctx);

    match op {
        Operation::Add => format!("{left} + {right}"),
        Operation::Subtract => format!("{left} - {right}"),
        Operation::Multiply => format!("{left} * {right}"),
        Operation::Divide => format!("{left} / {right}"),
        Operation::Modulo => format!("{left} % {right}"),
        Operation::Equal => format!("{left} == {right}"),
        Operation::NotEqual => format!("{left} != {right}"),
        Operation::Greater => format!("{left} > {right}"),
        Operation::GreaterEqual => format!("{left} >= {right}"),
        Operation::Less => format!("{left} < {right}"),
        Operation::LessEqual => format!("{left} <= {right}"),
        Operation::And => format!("{left} && {right}"),
        Operation::Or => format!("{left} || {right}"),
        Operation::Not => format!("!{left}"),
    }
}
