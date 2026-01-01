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
    helper::{get_expr_type, is_semicolon_needed, map_type},
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
                if is_semicolon_needed(&expr) {
                    str_body.push_str(&transpile_expr(expr, ctx));
                    str_body.push(';');
                } else {
                    str_body.push_str(&transpile_expr(expr, ctx));
                }
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
        Expr::Else { then_branch } => {
            let mut str_body = String::new();
            for expr in then_branch {
                if is_semicolon_needed(&expr) {
                    str_body.push_str(&transpile_expr(expr, ctx));
                    str_body.push(';');
                } else {
                    str_body.push_str(&transpile_expr(expr, ctx));
                }
            }
            format!("{str_body} ")
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
        Expr::List(list) => {
            let mut str_list = String::new();
            let str_type = map_type(&get_expr_type(&list[0]), true);
            for expr in list {
                str_list.push_str(&transpile_expr(expr, ctx));
                str_list.push(',');
            }
            str_list.pop();
            format!("[_]{str_type}{{{str_list}}}")
        }
        Expr::Assignment {
            name,
            value,
            symbol: _,
        } => {
            let transpiled = transpile_expr(*value, ctx);
            format!("{name} = {transpiled} ")
        }
        Expr::Return(value) => {
            let transpiled_value = transpile_expr(*value, ctx);

            format!("return {transpiled_value}")
        }

        other => {
            println!("{:?}", other);
            panic!("Error")
        }
    }
}
