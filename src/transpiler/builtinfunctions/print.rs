use crate::{
    parser::ast::{Expr, TemplateChunk},
    transpiler::{
        TranspileContext,
        helpers::{get_expr_type, get_format_str_from_type},
        transpile::transpile_expr,
    },
};

pub fn transpile_print<'a>(expr: &Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
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
                        let format_str = get_format_str_from_type(&typ, ctx.is_used_allocator);

                        format_parts.push_str(format_str);

                        let arg_code = transpile_expr(inner_expr, ctx);
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
            format!("std.debug.print(\"{format_parts}\\n\"{args_code})")
        }
        _ => {
            let my_type = get_expr_type(expr);
            let format_str = get_format_str_from_type(&my_type, ctx.is_used_allocator);
            let arg_code = transpile_expr(expr, ctx);
            ctx.uses_stdout = true;

            format!("std.debug.print(\"{format_str}\\n\", .{{{arg_code}}})")
        }
    }
}
