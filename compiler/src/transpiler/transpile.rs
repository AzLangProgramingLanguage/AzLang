use parser::{ast::Expr, shared_ast::BuiltInFunction};

use crate::transpiler::{
    TranspileContext,
    builtin::{
        min_max::{transpile_max, transpile_min},
        print::transpile_print,
        sum::transpile_sum,
    },
};

pub fn transpile_expr<'a>(expr: &'a Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
    match expr {
        Expr::String(s, b) => {
            if *b {
                format!("try allocator.dupe(u8, \"{}\")", s.escape_default())
            } else {
                format!("\"{}\"", s.escape_default())
            }
        }
        Expr::Number(n) => n.to_string(),
        Expr::Float(n) => n.to_string(),
        Expr::Bool(n) => n.to_string(),
        Expr::Break => "break".to_string(),
        Expr::Continue => "continue".to_string(),
        /*    Expr::Return(expr)=> {
            /* BUG:  Hell et burayı */
            let code = match &**expr {
                Expr::Number(n) => match get_expr_type(expr) {
                    Type::Natural => format!("azlangEded{{.natural={}}}", n),
                    Type::Integer => format!("azlangEded{{.integer={}}}", n),
                    Type::Float => format!("azlangEded{{.float={}}}", n),
                    _ => transpile_expr(expr, ctx),
                },

                Expr::VariableRef {
                    symbol,
                    ..
                } => {
                    let name = transpiled_name.as_deref().unwrap_or("<undefined>");
                    match symbol.as_ref().map(|s| &s.typ) {
                        Some(Type::Natural) => format!("{}", name),
                        Some(Type::Integer) => format!("{}", name),
                        Some(Type::Float) => format!("{}", name),
                        _ => transpile_expr(expr, ctx),
                    }
                }

                Expr::Float(n) => n.to_string(),
                Expr::String(s, _) => format!("\"{}\"", s.escape_default()),

        } */
        Expr::BuiltInCall { function, args, .. } => match function {
            BuiltInFunction::Print => transpile_print(&args[0], ctx),
            BuiltInFunction::Sum => transpile_sum(&args, ctx),

            BuiltInFunction::Min => transpile_min(&args, ctx),
            BuiltInFunction::Max => transpile_max(&args, ctx),

            _ => "None".to_string(),
        },
        _ => "None".to_string(),
    }
}
