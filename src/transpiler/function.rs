use crate::{
    context::{FunctionInfo, Parameter, Symbol, TranspileContext},
    expr::transpile_expr,
    parser::{Expr, ast::Type, builtin::match_builtin},
    transpiler::utils::map_type,
};

pub fn transpile_function_call(
    name: &str,
    _args: &[Expr], // İndi buna ehtiyac yoxdur
    resolved_params: &[Parameter],
    _return_type: Option<Type>,
    _ctx: &mut TranspileContext, // Kontekstə də ehtiyac yoxdur əgər `transpile_expr` çağırmırıqsa
) -> Result<String, String> {
    let args_code: Vec<String> = resolved_params
        .iter()
        .map(|param| {
            if param.is_pointer {
                format!("&{}", param.name)
            } else {
                param.name.clone()
            }
        })
        .collect();

    Ok(format!("{}({})", name, args_code.join(", ")))
}

pub fn transpile_function_def(
    name: &str,
    params: &[Parameter],
    body: &[Expr],
    return_type: Option<Type>,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    // Parametrləri transpile et
    let params_str: Vec<String> = params
        .iter()
        .map(|param| {
            let zig_type = map_type(&param.typ, !param.is_mutable);
            if param.is_mutable {
                format!("{}: *{}", param.name, zig_type)
            } else {
                format!("{}: {}", param.name, zig_type)
            }
        })
        .collect();

    // Geri dönüş tipi
    let ret_type = return_type.clone().unwrap_or(Type::Void);
    let ret_type_str = map_type(&ret_type, true);

    // Bədəni transpile et
    let mut body_lines = Vec::new();
    for expr in body {
        let mut line = transpile_expr(expr, ctx)?; // ✨ ctx artıq lazım deyil
        if !line.trim_end().ends_with(';') && !line.trim_start().starts_with("//") {
            line.push(';');
        }
        body_lines.push(format!("    {}", line));
    }

    // Funksiya kodunu qaytar
    Ok(format!(
        "fn {}({}) {} {{\n{}\n}}",
        name,
        params_str.join(", "),
        ret_type_str,
        body_lines.join("\n")
    ))
}
