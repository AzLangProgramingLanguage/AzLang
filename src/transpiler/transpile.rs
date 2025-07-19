use crate::{
    parser::ast::{BuiltInFunction, Expr},
    transpiler::{TranspileContext, builtinfunctions::print::transpile_print},
};

pub fn transpile_expr(expr: &Expr, ctx: &mut TranspileContext) -> String {
    match expr {
        Expr::String(s) => format!("\"{}\"", s.escape_default()),
        Expr::Number(n) => n.to_string(),
        Expr::Float(n) => n.to_string(),
        Expr::Bool(b) => b.to_string(),
        Expr::Break => "break".to_string(),
        Expr::Continue => "continue".to_string(),

        Expr::Return(expr) => {
            let arg_code = transpile_expr(expr, ctx);
            format!("return {arg_code}")
        }
        Expr::VariableRef { name, symbol } => {
            if ctx
                .enum_defs
                .values()
                .any(|variants| variants.contains(&name.to_string()))
            {
                format!(".{name}")
            } else if let Some(sym) = symbol {
                if sym.is_pointer {
                    format!("{name}.*")
                } else {
                    format!("{name}")
                }
            } else {
                format!("{name}")
            }
        }
        Expr::List(items) => {
            let items_code: Vec<String> =
                items.iter().map(|item| transpile_expr(item, ctx)).collect();
            let items_str = items_code.join(", ");
            format!("[{items_str}]")
        }
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => match function {
            BuiltInFunction::Print => transpile_print(&args[0], ctx),
            BuiltInFunction::Sqrt => {
                format!("@sqrt({}.0)", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Round => {
                format!("@round({})", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Floor => {
                format!("@floor({})", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Ceil => {
                format!("@ceil({})", transpile_expr(&args[0], ctx))
            }
            BuiltInFunction::Mod => {
                format!("@abs({})", transpile_expr(&args[0], ctx))
            }
            _ => todo!(),
        },

        _ => {
            println!("not yet implemented");
            println!("{:?}", expr);
            todo!()
        }
    }
}
