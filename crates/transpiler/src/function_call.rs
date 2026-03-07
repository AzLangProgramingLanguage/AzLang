use parser::{ast::Expr, shared_ast::Type};

use crate::{TranspileContext, transpile::transpile_expr};

pub fn transpile_function_call(
    ctx: &mut TranspileContext,
    target: Option<Box<Expr>>,
    name: Box<Expr>,
    args: Vec<Expr>,
    returned_type: Option<Type>,
) -> String {
    let name = transpile_expr(*name, ctx);
    let mut args_code = String::new();
    for arg in args {
        match &arg {
            Expr::VariableRef { name, symbol } => {
                args_code.push('&');
            }
            _ => {}
        }
        args_code.push_str(&transpile_expr(arg, ctx));
        args_code.push(',');
    }
    args_code.pop();

    if let Some(function) = ctx.functions.get(&name) {
        if function.is_used_try {
            return format!("try {name}({args_code})");
        } else {
            return format!("{name}({args_code})");
        }
    }
    format!("{name}({args_code})")
}
