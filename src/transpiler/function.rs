use crate::{
    Parameter,
    context::TranspileContext,
    expr::transpile_expr,
    parser::{Expr, ast::Type},
    transpiler::utils::map_type,
};

pub fn transpile_function_call(
    name: &str,
    args: &[Expr],
    _return_type: Option<Type>,
    ctx: &mut TranspileContext, // lazım ola bilər transpile_expr üçün
) -> Result<String, String> {
    let mut args_code = vec![];

    for arg in args {
        match arg {
            Expr::VariableRef {
                name,
                symbol: Some(sym),
            } => {
                println!("Symboool  {:?}", sym);
                if sym.is_pointer {
                    args_code.push(format!("&{}", name));
                } else {
                    args_code.push(name.clone());
                }
            }
            _ => {
                // Digər hallarda transpile_expr çağır
                let code = transpile_expr(arg, ctx)?;
                args_code.push(code);
            }
        }
    }

    Ok(format!("{}({})", name, args_code.join(", ")))
}

pub fn transpile_function_def(
    name: &str,
    params: &[Parameter],
    body: &[Expr],
    return_type: &Option<Type>,
    _parent: &Option<String>,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    let params_str: Vec<String> = params.iter().map(transpile_param).collect();

    let ret_type = return_type.clone().unwrap_or(Type::Void);
    let ret_type_str = map_type(&ret_type, true);

    let mut body_lines = Vec::new();
    for expr in body {
        let mut line = transpile_expr(expr, ctx)?;
        if is_semicolon_needed(expr) && !line.trim_start().starts_with("//") {
            line.push(';');
        }
        body_lines.push(format!("    {}", line));
    }

    Ok(format!(
        "fn {}({}) {} {{\n{}\n}}",
        name,
        params_str.join(", "),
        ret_type_str,
        body_lines.join("\n")
    ))
}

pub fn is_semicolon_needed(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Assignment { .. }
            | Expr::Break
            | Expr::Continue
            | Expr::Return(_)
            | Expr::MutableDecl { .. }
            | Expr::ConstantDecl { .. }
            | Expr::FunctionCall { .. }
            | Expr::BuiltInCall { .. }
            | Expr::MethodCall { .. }
            | Expr::VariableRef { .. }
            | Expr::FieldAccess { .. }
            | Expr::Index { .. }
            | Expr::BinaryOp { .. }
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
