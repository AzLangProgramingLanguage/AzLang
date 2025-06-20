use crate::{
    context::TranspileContext,
    expr::transpile_expr,
    parser::{Expr, Program},
};

pub fn generate_main_body(program: &Program, ctx: &mut TranspileContext) -> Result<String, String> {
    let mut body = String::new();

    for expr in &program.expressions {
        if matches!(expr, Expr::FunctionDef { .. }) {
            continue; // top_level tərəfindən işləndi
        }

        let line = transpile_expr(expr, ctx)?;
        let line = if line.trim_end().ends_with(';') || line.is_empty() {
            line
        } else {
            format!("{};", line)
        };
        body.push_str("    ");
        body.push_str(&line);
        body.push('\n');
    }

    Ok(body)
}
