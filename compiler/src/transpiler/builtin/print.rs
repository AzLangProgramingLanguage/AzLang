use parser::{ast::Expr, ast::TemplateChunk, shared_ast::Type};

use crate::transpiler::{
    TranspileContext,
    definition::function_def::is_muttable,
    helper::{get_expr_type, get_format_str_from_type},
    transpile::transpile_expr,
};

fn arg_code_for_expr<'a>(
    expr: &'a Expr<'a>,
    ctx: &mut TranspileContext<'a>,
    typ: Type<'_>,
) -> String {
    match expr {
        Expr::String(_)
        | Expr::TemplateString(_)
        | Expr::Number(_)
        | Expr::List(_)
        | Expr::Float(_)
        | Expr::UnaryOp { op: _, expr: _ } => {
            return transpile_expr(expr, ctx);
        }

        _ => {}
    }

    match typ {
        Type::String => {
            let name = transpile_expr(expr, ctx);
            if is_muttable(expr) {
                /* TODO: buraya baxarsan */
                format!("{}.Mut", name)
            } else {
                format!("{}.Const", name)
            }
        }
        Type::LiteralConstString => transpile_expr(expr, ctx),
        Type::Natural | Type::Integer => {
            let name = transpile_expr(expr, ctx);

            format!("toInteger({})", name)
        }
        Type::Float => {
            let name = transpile_expr(expr, ctx);
            format!("toFloat({})", name)
        }

        _ => transpile_expr(expr, ctx),
    }
}

pub fn transpile_print<'a>(expr: &'a Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
    // ctx.uses_stdout = true;

    match expr {
        Expr::TemplateString(chunks) => {
            let mut format_parts = String::new();
            let mut args = Vec::new();

            for chunk in chunks {
                match chunk {
                    TemplateChunk::Literal(s) => {
                        format_parts.push_str(&s.replace('\\', "\\\\").replace('"', "\\\""));
                    }
                    TemplateChunk::Expr(inner_expr) => {
                        let typ = get_expr_type(inner_expr);
                        format_parts.push_str(get_format_str_from_type(
                            &typ, false, /*TODO: Check:1 ctx.is_used_allocator */
                        ));
                        args.push(arg_code_for_expr(inner_expr, ctx, typ));
                    }
                }
            }

            let args_code = if args.is_empty() {
                String::new()
            } else {
                format!(", .{{ {} }}", args.join(", "))
            };

            format!("std.debug.print(\"{format_parts}\\n\"{args_code})")
        }

        // Sadə expression variantı
        _ => {
            let typ: Type<'_> = get_expr_type(expr);
            let is_allocator = false; /*  {
            if let Expr::Call {
            target: _,
            name: _,
            args: _,
            returned_type: _,
            is_allocator,
            } = expr
            {
            is_allocator
            } else {
            &false
            }
            }; */

            let format_str = get_format_str_from_type(&typ, is_allocator);
            let arg_code = arg_code_for_expr(expr, ctx, typ);

            format!("std.debug.print(\"{format_str}\\n\", .{{{arg_code}}})")
        }
    }
}
