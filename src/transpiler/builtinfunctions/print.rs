use std::borrow::Cow;

use crate::{
    parser::ast::{Expr, TemplateChunk, Type},
    transpiler::{
        TranspileContext,
        helpers::{get_expr_type, get_format_str_from_type, is_muttable},
        transpile::transpile_expr,
    },
};

fn arg_code_for_expr<'a>(
    expr: &'a Expr<'a>,
    ctx: &mut TranspileContext<'a>,
    typ: Type<'_>,
) -> String {
    println!("{:#?}", expr);
    match expr {
        Expr::String(_, _)
        | Expr::TemplateString(_)
        | Expr::Number(_)
        | Expr::UnaryOp { op: _, expr: _ } => {
            return transpile_expr(expr, ctx);
        }
        _ => {}
    }
    match typ {
        Type::Metn => {
            let name = transpile_expr(expr, ctx);
            if is_muttable(expr) {
                format!("{}.Mut", name)
            } else {
                format!("{}.Const", name)
            }
        }
        Type::Natural => {
            let name = transpile_expr(expr, ctx);

            format!("{}.natural", name)
        }
        Type::Float => {
            let name = transpile_expr(expr, ctx);
            format!("{}.float", name)
        }
        Type::Integer => {
            let name = transpile_expr(expr, ctx);
            format!("{}.integer", name)
        }

        _ => transpile_expr(expr, ctx),
    }
}

pub fn transpile_print<'a>(expr: &'a Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
    ctx.uses_stdout = true;

    match expr {
        // Template string variantı
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
                        format_parts
                            .push_str(get_format_str_from_type(&typ, ctx.is_used_allocator));
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
            let is_allocator = {
                if let Expr::Call {
                    target: _,
                    name: _,
                    transpiled_name: _,
                    args: _,
                    returned_type: _,
                    is_allocator,
                } = expr
                {
                    is_allocator
                } else {
                    &false
                }
            };
            let format_str = get_format_str_from_type(&typ, *is_allocator);
            let arg_code = arg_code_for_expr(expr, ctx, typ);
            format!("std.debug.print(\"{format_str}\\n\", .{{{arg_code}}})")
        }
    }
}
