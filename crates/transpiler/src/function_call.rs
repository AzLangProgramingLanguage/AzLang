use crate::{TranspileContext, transpile_expr};
use parser::shared_ast::Type;
use std::fmt::Write;
use validator::ast::Expr;

pub fn transpile_function_call(
    buf: &mut String,
    ctx: &mut TranspileContext,
    name: Expr,
    args: Vec<Expr>,
) {
    let mut function_name = String::new();
    transpile_expr(name, ctx, &mut function_name);

    let function = ctx.functions.get(&function_name).unwrap().clone();
    buf.push_str(&function_name);
    buf.push('(');

    for (i, arg) in args.into_iter().enumerate() {
        if i > 0 {
            buf.push(',');
        }
        if function.params[i].typ == Type::Any {
            match &arg {
                Expr::String(s) => {
                    write!(buf, "ValueType{{ .str = {s}  }}");
                }
                Expr::Number(i) => {
                    write!(buf, "ValueType{{ .int = {i}  }}");
                }
                Expr::Float(f) => {
                    write!(buf, "ValueType{{ .float = {f}  }}");
                }
                Expr::Bool(b) => {
                    write!(buf, "ValueType{{ .bool = {b}  }}");
                }
                Expr::VariableRef { name, symbol } => match &symbol.typ {
                    Type::Integer => {
                        write!(buf, "ValueType{{ .int = {name}  }}");
                    }
                    Type::Float => {
                        write!(buf, "ValueType{{ .float = {name}  }}");
                    }
                    Type::String(strenum) => {
                        write!(buf, "ValueType{{ .str = {name}  }}");
                    }
                    _ => panic!("Burası hele hazir deyil"),
                },
                _ => panic!("Burası hele hazır deyil"),
            }
        }
        match arg {
            Expr::VariableRef { name, .. } => {
                buf.push('&');
                buf.push_str(&name);
            }

            other => {
                transpile_expr(other, ctx, buf);
            }
        }
    }
    buf.push(')');
}
