use crate::{TranspileContext, transpile_expr};
use parser::shared_ast::Type;
use std::fmt::Write;
use validator::ast::Expr;

pub fn transpile_function_call(
    buf: &mut String,
    ctx: &mut TranspileContext,
    name: Expr,
    args: Vec<Expr>,
) {
    transpile_expr(name, ctx, buf);
    buf.push('(');

    for (i, arg) in args.into_iter().enumerate() {
        if i > 0 {
            buf.push(',');
        }
        match arg {
            Expr::VariableRef { name, .. } => {
                buf.push('&');
                buf.push_str(&name);
            }

            other => {
                transpile_expr(other, ctx, buf);
            }
        }
    }
    buf.push(')');
}
