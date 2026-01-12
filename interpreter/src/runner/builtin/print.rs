use crate::runner::Runner;
use crate::runner::runner::runner_interpretator;
use parser::ast::{Expr, TemplateChunk};
use std::fmt::Write;

pub fn print_interpreter<'a>(expr: Expr<'a>, ctx: &mut Runner<'a>) -> String {
    let mut output = String::new();
    match expr {
        Expr::TemplateString(chunks) => {
            for chunk in chunks {
                match chunk {
                    TemplateChunk::Literal(s) => output.push_str(&s),
                    TemplateChunk::Expr(inner_expr) => {
                        let new_expr = *inner_expr;

                        let evaluated = runner_interpretator(ctx, new_expr);
                        exporter_to_string(&evaluated, ctx, &mut output);
                    }
                }
            }
        }
        _ => {
            let evaluated = runner_interpretator(ctx, expr.clone());
            exporter_to_string(&evaluated, ctx, &mut output);
        }
    }

    output
}

fn exporter_to_string(expr: &Expr, ctx: &Runner, out: &mut String) {
    match expr {
        Expr::String(s) => out.push_str(s),
        Expr::DynamicString(s) => out.push_str(s),
        Expr::Number(n) => {
            let _ = write!(out, "{}", n);
        }

        Expr::Float(f) => {
            let _ = write!(out, "{}", f);
        }

        Expr::Bool(b) => {
            out.push_str(if *b { "doğru" } else { "yanlış" });
        }

        Expr::Char(c) => out.push(*c),

        Expr::StructInit { args, .. } => {
            out.push('{');
            let mut first = true;
            for (name, value) in args {
                if !first {
                    out.push_str(", ");
                }
                first = false;
                let _ = write!(out, "{}: ", name);
                exporter_to_string(value, ctx, out);
            }
            out.push('}');
        }

        Expr::VariableRef { name, .. } => {
            if let Some(var) = ctx.variables.get(&name.to_string()) {
                exporter_to_string(&var.value, ctx, out);
            } else {
                let _ = write!(out, "<undef:{}>", name);
            }
        }

        Expr::BinaryOp {
            left, right, op, ..
        } => {
            out.push('(');
            exporter_to_string(left, ctx, out);
            let _ = write!(out, " {} ", op);
            exporter_to_string(right, ctx, out);
            out.push(')');
        }

        Expr::BuiltInCall { function, args, .. } => {
            let _ = write!(out, "{}(", function);
            let mut first = true;
            for arg in args {
                if !first {
                    out.push_str(", ");
                }
                first = false;
                exporter_to_string(arg, ctx, out);
            }
            out.push(')');
        }

        Expr::List(list) => {
            out.push('[');
            let mut first = true;
            for e in list {
                if !first {
                    out.push_str(", ");
                }
                first = false;
                exporter_to_string(e, ctx, out);
            }
            out.push(']');
        }

        _ => {
            println!("Unknown type: {:?}", expr);
            out.push_str("<unknown>")
        }
    }
}
