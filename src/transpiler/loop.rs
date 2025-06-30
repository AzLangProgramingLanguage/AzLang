use crate::context::TranspileContext;
use crate::function::is_semicolon_needed;
use crate::parser::Expr;
use crate::transpiler::expr::transpile_expr;

pub fn transpile_loop(
    var_name: &str,
    iterable: &Expr,
    body: &[Expr],
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    let iterable_code = transpile_expr(iterable, ctx)?;

    let mut body_lines = Vec::new();
    for expr in body {
        let mut line = transpile_expr(expr, ctx)?;
        if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
            line.push(';');
        }
        body_lines.push(format!("    {}", line));
    }

    let body_code = body_lines.join("\n");

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
