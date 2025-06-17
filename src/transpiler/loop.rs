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
        .collect::<Result<Vec<String>, String>>()?;

    // Əgər iterable `VariableRef`-dirsə və mutable siyahıdırsa `.items` istifadə et
    if let Expr::VariableRef(name) = iterable {
        if ctx.mutable_symbols.contains(name) {
            return Ok(format!(
                "for ({}.items) |{}| {{\n{}\n}}",
                iterable_code,
                var_name,
                body_code.join("\n")
            ));
        }
    }

    // Normal dövr (buraya `range(start, end)` də daxil ola bilər)
    Ok(format!(
        "for ({}) |{}| {{\n{}\n}}",
        iterable_code,
        var_name,
        body_code.join("\n")
    ))
}
