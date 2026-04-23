use parser::{
    ast::{Expr, TemplateChunk},
    shared_ast::Type,
};

use crate::{
    TranspileContext,
    helper::{get_expr_type, get_format_str_from_type, is_primite_value},
    transpile_expr,
};
pub fn transpile_print(expr: Expr, ctx: &mut TranspileContext) -> String {
    let mut format_parts = String::new();
    let mut args: Vec<String> = Vec::new();

    ctx.imports
        .insert("const std = @import(\"std\");".to_string());
    match expr {
        Expr::TemplateString(chunks) => {
            for chunk in chunks {
                match chunk {
                    TemplateChunk::Literal(s) => {
                        format_parts.push_str(&s);
                    }
                    TemplateChunk::Expr(inner_expr) => {
                        let typ = get_expr_type(&inner_expr);
                        format_parts.push_str(get_format_str_from_type(&typ));
                        args.push(transpile_expr(*inner_expr, ctx));
                    }
                }
            }
            format!(
                "std.debug.print(\"{format_parts}\\n\",.{{{}}})",
                args.join(" , ")
            )
        }

        _ => {
            if is_primite_value(&expr) {
                // ctx.used_try = true;
                match expr {
                    Expr::Bool(true) => {
                        String::from("try std.fs.File.stdout().writeAll(\"doğrudur\n\")")
                    }
                    Expr::Bool(false) => {
                        String::from("try std.fs.File.stdout().writeAll(\"yanlışdır\n\")")
                    }
                    _ => {
                        format!(
                            "try std.fs.File.stdout().writeAll(\"{}\\n\")",
                            transpile_expr(expr, ctx).trim_matches('"')
                        )
                    }
                }
            } else {
                let typ = get_expr_type(&expr);
                let transpiled = transpile_expr(expr, ctx);
                ctx.add_import(
                    "  const to_string = @import(\"./dependencies/to_string.zig\").to_string; ",
                );
                if matches!(typ, Type::Array(_)) {
                    // ctx.needs_allocator = true;
                    format!(
                        "std.debug.print(\"{{s}}\\n\",.{{ (try to_string(@TypeOf({}),allocator,{})) }})",
                        transpiled, transpiled
                    )
                } else {
                    format!(
                        "std.debug.print(\"{}\\n\",.{{{}}})",
                        get_format_str_from_type(&typ),
                        transpiled
                    )
                }
            }
        }
    }
}
