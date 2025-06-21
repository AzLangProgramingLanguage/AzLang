use crate::{
    context::TranspileContext,
    expr::transpile_expr,
    parser::{Expr, Program},
};

pub fn generate_main_body(program: &Program, ctx: &mut TranspileContext) -> Result<String, String> {
    let mut body = String::new();

    for expr in &program.expressions {
        if matches!(expr, Expr::FunctionDef { .. }) {
            continue;
        }

        let line = transpile_expr(expr, ctx)?;

        let needs_semicolon = matches!(
            expr,
            Expr::Assignment { .. }
                | Expr::Break
                | Expr::Continue
                | Expr::MutableDecl { .. }
                | Expr::ConstantDecl { .. }
                | Expr::FunctionCall { .. }
                | Expr::BuiltInCall { .. }
                | Expr::MethodCall { .. }
                | Expr::VariableRef(_)
                | Expr::FieldAccess { .. }
                | Expr::Index { .. }
                | Expr::BinaryOp { .. }
        );

        let line = if needs_semicolon && !line.trim_end().ends_with(';') {
            format!("{};", line)
        } else {
            line
        };

        body.push_str("    ");
        body.push_str(&line);
        body.push('\n');
    }

    Ok(body)
}
