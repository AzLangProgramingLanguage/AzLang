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
        .collect::<Result<Vec<String>, String>>()?
        .join("\n");

    if let Expr::VariableRef(name) = iterable {
        if let Some(symbol) = ctx.lookup_variable(name) {
            if symbol.is_mutable {
                return Ok(format!(
                    "for ({}.items) |{}| {{\n{}\n}}",
                    iterable_code, var_name, body_code
                ));
            }
        }
    }

    Ok(format!(
        "for ({}) |{}| {{\n{}\n}}",
        iterable_code, var_name, body_code
    ))
}
