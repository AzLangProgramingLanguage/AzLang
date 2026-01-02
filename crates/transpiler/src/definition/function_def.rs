use parser::{
    ast::{Expr, Parameter},
    shared_ast::Type,
};

use crate::{
    FunctionDef, TranspileContext,
    helper::{is_semicolon_needed, map_type},
    transpile::transpile_expr,
};

pub fn transpile_function_def<'a>(
    name: &'a str,
    params: Vec<Parameter<'a>>,
    body: Vec<Expr<'a>>,
    return_type: &Option<Type<'_>>,
    _parent: Option<&'a str>,
    ctx: &mut TranspileContext<'a>,
    _is_allocator: &bool,
) -> String {
    let mut new_str = String::new();
    let params_str: String = {
        for param in params {
            new_str.push_str(transpile_param(&param).as_ref());
            new_str.push(',');
        }
        new_str.pop(); 
        new_str
    };
    let ret_type = return_type.as_ref().unwrap_or(&Type::Void);
    let ret_type_str = map_type(ret_type, true);

    let mut body_lines = String::new();
    for expr in body.into_iter() {
        if is_semicolon_needed(&expr) {
            body_lines.push_str(&format!("    {};", transpile_expr(expr, ctx)));
        } else {
            body_lines.push_str(&format!("    {}", transpile_expr(expr, ctx)));
        }
    }
    if ctx.used_try {
        ctx.functions
            .insert(name.to_string(), FunctionDef { is_used_try: true });
        ctx.used_try = false;
        return format!(
            "fn {}({}) !{} {{ {} }}",
            name, params_str, ret_type_str, body_lines
        );
    }
    format!(
        "fn {}({}) {} {{ {} }}",
        name, params_str, ret_type_str, body_lines
    )
}

fn transpile_param(param: &Parameter) -> String {
    let zig_type = map_type(&param.typ, !param.is_mutable);
    if param.is_mutable {
        format!("{}: *{}", param.name, zig_type)
    } else {
        format!("{}: {}", param.name, zig_type)
    }
}
