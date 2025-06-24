use crate::{
    context::{FunctionInfo, Parameter, Symbol, TranspileContext},
    expr::transpile_expr,
    parser::{Expr, ast::Type, builtin::match_builtin},
    transpiler::utils::map_type,
};

pub fn transpile_function_call(
    name: &str,
    args: &[Expr],
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    // Normal funksiya çağırışı
    // Yeni simvollar varsa default olaraq Metn (string) tipində əlavə et
    for arg in args {
        if let Expr::VariableRef(var_name) = arg {
            if !ctx.symbol_types.contains_key(var_name) {
                ctx.declare_variable(
                    var_name.clone(),
                    Symbol {
                        typ: Type::Metn,
                        is_mutable: false,
                        is_used: false,
                        is_param: false,
                        source_location: None,
                    },
                );
            }
        }
    }

    let args_code = args
        .iter()
        .map(|arg| transpile_expr(arg, ctx))
        .collect::<Result<Vec<_>, _>>()?;

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
            format!("{}: {}", param.name, zig_type)
        })
        .collect();

    // Geri dönüş tipi
    let ret_type = return_type.clone().unwrap_or(Type::Void);
    let ret_type_str = map_type(&ret_type, true);

    // ✅ Yeni scope aç
    ctx.push_scope();

    // ✅ Parametrləri context-ə əlavə et
    for param in params {
        let symbol = Symbol {
            typ: param.typ.clone(),
            is_mutable: param.is_mutable,
            is_used: false,
            is_param: true,
            source_location: None,
        };
        ctx.declare_variable(param.name.clone(), symbol);
    }

    // ✅ Bədəni transpile et
    let mut body_lines = Vec::new();
    for expr in body {
        let mut line = transpile_expr(expr, ctx)?;
        if !line.trim_end().ends_with(';') && !line.trim_start().starts_with("//") {
            line.push(';');
        }
        body_lines.push(format!("    {}", line));
    }

    // ✅ Scope-u təmizlə
    ctx.pop_scope();

    // ✅ Funksiyanı yadda saxla
    ctx.declare_function(FunctionInfo {
        name: name.to_string(),
        return_type,
        parameters: params.to_vec(),
        body: None,
        scope_level: ctx.scopes.len(),
        is_public: false,
    });

    // ✅ Zig kodu qaytar
    Ok(format!(
        "fn {}({}) {} {{\n{}\n}}",
        name,
        params_str.join(", "),
        ret_type_str,
        body_lines.join("\n")
    ))
}
