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
            buf.push('&');
            let mut val = String::new();

            match arg {
                Expr::String(s) => {
                    write!(buf, "ValueType{{ .tag=ValueTag.str, .data = {s}  }}").unwrap();
                }
                Expr::Number(i) => {
                    write!(buf, "ValueType{{ .tag=ValueTag.int,.data = {i}  }}").unwrap();
                }
                Expr::Float(f) => {
                    write!(buf, "ValueType{{ .tag=ValueTag.float,.data = {f}  }}").unwrap();
                }
                Expr::Bool(b) => {
                    write!(buf, "ValueType{{ .tag=ValueTag.bool, .data = {b}  }}").unwrap();
                }
                Expr::VariableRef { name, symbol } => {
                    transpile_type(symbol.typ, buf, name);
                }
                Expr::BinaryOp {
                    left,
                    right,
                    op,
                    return_type,
                } => {
                    transpile_expr(
                        Expr::BinaryOp {
                            left,
                            right,
                            op,
                            return_type: return_type.clone(),
                        },
                        ctx,
                        &mut val,
                    );
                    transpile_type(return_type, buf, val);
                }
                other => panic!("Burası hele hazır deyil {other:?}"),
            }
        } else {
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
    }
    buf.push(')');
}
fn transpile_type(typ: Type, buf: &mut String, val: String) {
    match typ {
        Type::Integer => {
            write!(buf, "ValueType{{ .tag=ValueTag.int,.data = {val}  }}").unwrap();
        }
        Type::Float => {
            write!(buf, "ValueType{{ .tag=ValueTag.float, .data={val}  }}").unwrap();
        }
        Type::String(strenum) => {
            write!(buf, "ValueType{{ .tag=ValueTag.str, .data = {val}  }}").unwrap();
        }
        other => panic!("Burası hele hazir deyil {other}"),
    }
}
