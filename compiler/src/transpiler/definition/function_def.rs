use parser::{
    ast::{Expr, Parameter},
    shared_ast::Type,
};

use crate::transpiler::{TranspileContext, helper::map_type};
/* BUG: Burasında boşluq var */

pub fn transpile_function_def<'a>(
    name: &'a str,
    params: &'_ [Parameter<'a>],
    body: &'a [Expr<'a>],
    return_type: &Option<Type<'_>>,
    _parent: Option<&'a str>,
    ctx: &mut TranspileContext<'a>,
    is_allocator: &bool,
) -> String {
    /*  let params_str: Vec<String> = params.iter().map(transpile_param).collect();

    let ret_type = return_type.as_ref().unwrap_or(&Type::Void);
    let ret_type_str = map_type(ret_type, true);

    let mut body_lines = Vec::new();
    for expr in body {
        let mut line = transpile_expr(expr, ctx);
        if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
            line.push(';');
        }
        body_lines.push(format!("    {}", line));
    } */
    let name = "asda";
    let params_str = "";
    let ret_type_str = "";
    let body_lines = "";
    format!(
        "fn {}({}) {} {{\n{}\n}}",
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

pub fn is_muttable<'a>(expr: &'a Expr<'a>) -> bool {
    match expr {
        Expr::VariableRef { name: _, symbol } => {
            if let Some(sym) = symbol {
                return sym.is_mutable;
            }
        }
        Expr::Call { target, .. } => match target {
            Some(boxed_expr) => match &**boxed_expr {
                Expr::VariableRef { name: _, symbol } => {
                    if let Some(sym) = symbol {
                        return sym.is_mutable;
                    }
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    }
    false
}
