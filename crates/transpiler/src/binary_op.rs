use parser::{ast::Expr, shared_ast::Type};

use crate::{TranspileContext, transpile::transpile_expr};

pub fn transpile_binary_op<'a>(
    ctx: &mut TranspileContext<'a>,
    left: Box<Expr<'a>>,
    right: Box<Expr<'a>>,
    op: &'a str,
    return_type: Type<'a>,
) -> String {
    let left = transpile_expr(*left, ctx);
    let right = transpile_expr(*right, ctx);

    format!("{left} {op} {right}")
}
