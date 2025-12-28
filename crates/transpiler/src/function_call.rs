use parser::{ast::Expr, shared_ast::Type};

use crate::{TranspileContext, transpile::transpile_expr};

pub fn transpile_function_call<'a>(
    ctx: &mut TranspileContext<'a>,
    target: Option<Box<Expr<'a>>>,
    name: &'a str,
    args: Vec<Expr<'a>>,
    returned_type: Option<Type<'a>>,
) -> String {
    let mut args_code = String::new();
    for arg in args {
        args_code.push_str(&transpile_expr(arg, ctx));
        args_code.push(',');
    }
    args_code.pop();

    if let Some(function) = ctx.functions.get(name) {
        if function.is_used_try {
            return format!("try {name}({args_code})");
        } else {
            return format!("{name}({args_code})");
        }
    }
    format!("{name}({args_code})")
}
