use crate::context::TranspileContext;
use crate::parser::Expr;
use crate::transpiler::expr::transpile_expr;

pub fn transpile_loop(
    var_name: &str,
    iterable: &Expr,
    body: &[Expr],
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    let iterable_code = transpile_expr(iterable, ctx)?;
    let body_code = body
        .iter()
        .map(|stmt| transpile_expr(stmt, ctx))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");

    let loop_expr = match iterable {
        Expr::VariableRef {
            symbol: Some(sym), ..
        } => {
            if sym.is_mutable {
                format!("{}.items", iterable_code)
            } else {
                iterable_code.clone()
            }
        }
        _ => iterable_code.clone(),
    };

    Ok(format!(
        "for ({}) |{}| {{\n{}\n}}",
        loop_expr, var_name, body_code
    ))
}
