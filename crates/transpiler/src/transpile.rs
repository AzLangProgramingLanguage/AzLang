use parser::{ast::Expr, shared_ast::BuiltInFunction};

use crate::{
    TranspileContext,
    binary_op::transpile_binary_op,
    builtin::{
        min_max::{transpile_max, transpile_min},
        print::transpile_print,
        sum::transpile_sum,
    },
    declaration::variable_decl::transpile_decl,
    function_call::transpile_function_call,
};

pub fn transpile_expr<'a>(expr: Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
    match expr {
        Expr::String(s) => format!("{}", s.escape_default()),
        Expr::DynamicString(s) => format!("try allocator.dupe(u8, \"{}\")", s.escape_default()),
        Expr::Number(n) => n.to_string(),
        Expr::Float(n) => n.to_string(),
        Expr::Bool(n) => n.to_string(),
        Expr::Break => "break".to_string(),
        Expr::Continue => "continue".to_string(),
        Expr::VariableRef { name, symbol: _ } => name.to_string(),
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => transpile_function_call(ctx, target, name, args, returned_type),

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
        Expr::BinaryOp {
            left,
            right,
            op,
            return_type,
        } => transpile_binary_op(ctx, left, right, op, return_type),
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => transpile_decl(&name.to_string(), typ, is_mutable, *value, ctx),
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let str_condition = transpile_expr(*condition, ctx);
            let mut str_body = String::new();
            for expr in then_branch {
                str_body.push_str(&transpile_expr(expr, ctx));
            }
            if else_branch.is_empty() {
                format!("if({str_condition}) {{ {str_body} }}")
            } else {
                let mut str_else_body = String::new();
                for expr in else_branch {
                    str_else_body.push_str(&transpile_expr(expr, ctx));
                }
                format!("if({str_condition}) {{ {str_body} }}  else {{ {str_else_body} }}")
            }
        }
        Expr::BuiltInCall {
            function, mut args, ..
        } => match function {
            BuiltInFunction::Print => transpile_print(args.remove(0), ctx),
            BuiltInFunction::Sum => transpile_sum(&mut args, ctx),

            BuiltInFunction::Min => transpile_min(&mut args, ctx),
            BuiltInFunction::Max => transpile_max(&mut args, ctx),

            _ => "None".to_string(),
        },
        _ => "None".to_string(),
    }
}
