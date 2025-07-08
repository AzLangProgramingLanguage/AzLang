use crate::{
    context::TranspileContext,
    expr::transpile_expr,
    parser::{
        Expr,
        ast::{TemplateChunk, Type},
    },
    transpiler::utils::{get_expr_type, get_format_str_from_type},
};

pub fn transpile_builtin_print(
    expr: &Expr,
    resolved_type: &Option<Type>,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    match expr {
        Expr::TemplateString(chunks) => {
            let mut format_parts = String::new();
            let mut args = Vec::new();

            for chunk in chunks {
                match chunk {
                    TemplateChunk::Literal(s) => {
                        format_parts.push_str(&s.replace("\\", "\\\\").replace("\"", "\\\""));
                    }
                    TemplateChunk::Expr(inner_expr) => {
                        let typ = get_expr_type(inner_expr);
                        let format_str = typ
                            .as_ref()
                            .map(|t| get_format_str_from_type(t))
                            .unwrap_or("{any}");

                        format_parts.push_str(format_str);

                        let arg_code = transpile_expr(inner_expr, ctx)?;
                        args.push(arg_code);
                    }
                }
            }

            let args_code = if args.is_empty() {
                "".to_string()
            } else {
                format!(", .{{ {} }}", args.join(", "))
            };

            ctx.uses_stdout = true;
            Ok(format!(
                "std.debug.print(\"{}\\n\"{})",
                format_parts, args_code
            ))
        }

        _ => {
            let format_str = resolved_type
                .as_ref()
                .map(|t| get_format_str_from_type(&t))
                .unwrap_or("{any}");

            let arg_code = transpile_expr(expr, ctx)?;

            ctx.uses_stdout = true;
            Ok(format!(
                "std.debug.print(\"{}\\n\", .{{{}}})",
                format_str, arg_code
            ))
        }
    }
}
