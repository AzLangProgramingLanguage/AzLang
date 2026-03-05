use parser::ast::Expr;

use crate::{TranspileContext, transpile::transpile_expr};

pub fn transpile_len<'a>(args: &mut Vec<Expr>, ctx: &mut TranspileContext<'a>) -> String {
    let transpiled = transpile_expr(args.remove(0), ctx);
    format!("{transpiled}.len")
}
