use crate::{
    context::TranspileContext,
    expr::transpile_expr,
    parser::{Expr, ast::Type},
    transpiler::utils::{map_type, transpile_input_var},
};

pub fn transpile_mutable_decl(
    name: &str,
    typ: &Option<Type>,
    value: &Expr,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    if let Some(result) = transpile_special_case(name, typ, value, ctx, true) {
        return result;
    }

    let value_code = transpile_expr(value, ctx)?;
    let inferred_type = match typ {
        Some(t) => t.clone(),
        None => ctx
            .lookup_variable(name)
            .map(|sym| sym.typ)
            .ok_or_else(|| format!("Tip kontekstdə tapılmadı: '{}'", name))?,
    };
    if let Type::Istifadeci(ref enum_name) = inferred_type {
        if ctx.enum_defs.contains_key(enum_name) {
            // enum dəyişəni üçün variant nöqtə ilə yazılır
            if let Expr::VariableRef(variant_name) = value {
                return Ok(format!("var {} = {}.{};", name, enum_name, variant_name));
            }
        }
    }

    let decl = match &inferred_type {
        Type::Metn => {
            ctx.needs_allocator = true;
            format!(
                "var {}: []u8 = try allocator.dupe(u8, {})",
                name, value_code
            )
        }

        Type::Siyahi(inner) => {
            if let Expr::List(items) = value {
                let items_code: Result<Vec<_>, _> =
                    items.iter().map(|i| transpile_expr(i, ctx)).collect();
                let items_str = items_code?.join(", ");

                ctx.needs_allocator = true;
                ctx.cleanup_statements.push(format!("{}.deinit();", name));

                let inner_type = map_type(inner, false);
                format!(
                    r#"var {name} = try std.ArrayList({inner_type}).initCapacity(allocator, {cap});
try {name}.appendSlice(&[_]{inner_type}{{ {items_str} }});"#,
                    name = name,
                    cap = items.len(),
                    inner_type = inner_type,
                    items_str = items_str
                )
            } else {
                return Err("Siyahı tipli dəyişən üçün dəyər siyahı olmalıdır.".to_string());
            }
        }
        _ => {
            format!(
                "var {}: {} = {}",
                name,
                map_type(&inferred_type, false),
                value_code
            )
        }
    };

    Ok(decl)
}
pub fn transpile_constant_decl(
    name: &str,
    typ: &Option<Type>,
    value: &Expr,
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    if let Some(result) = transpile_special_case(name, typ, value, ctx, false) {
        return result;
    }

    let inferred_type = match typ {
        Some(t) => t.clone(),
        None => ctx
            .lookup_variable(name)
            .map(|sym| sym.typ)
            .ok_or_else(|| format!("Tip kontekstdə tapılmadı: '{}'", name))?,
    };

    if let Type::Istifadeci(ref enum_name) = inferred_type {
        if ctx.enum_defs.contains_key(enum_name) {
            // enum dəyişəni üçün variant nöqtə ilə yazılır
            if let Expr::VariableRef(variant_name) = value {
                return Ok(format!("const {} = {}.{};", name, enum_name, variant_name));
            }
        }
    }

    if let Expr::List(items) = value {
        let items_code: Result<Vec<String>, String> =
            items.iter().map(|item| transpile_expr(item, ctx)).collect();
        let items_str = items_code?.join(", ");

        let actual_type = typ.clone().unwrap_or_else(|| inferred_type.clone());

        if let Type::Siyahi(inner_type) = actual_type {
            if items.is_empty() {
                return Ok(format!(
                    "const {} = &[_]{}{{}}",
                    name,
                    map_type(&*inner_type, true)
                ));
            }

            return Ok(format!(
                "const {}: {} = &[_]{}{{ {} }}",
                name,
                map_type(&Type::Siyahi(inner_type.clone()), true),
                map_type(&*inner_type, true),
                items_str
            ));
        }

        if items.is_empty() && typ.is_none() {
            return Ok(format!("const {} = &{{}}", name));
        }
    }

    let value_code = transpile_expr(value, ctx)?;
    Ok(format!(
        "const {}: {} = {}",
        name,
        map_type(&inferred_type, true),
        value_code
    ))
}

fn is_input_expr(expr: &Expr) -> Option<&[Expr]> {
    match expr {
        Expr::BuiltInCall {
            func,
            args,
            resolved_type: _,
        } if matches!(func, crate::parser::ast::BuiltInFunction::Input) => Some(args),
        _ => None,
    }
}
pub fn transpile_special_case(
    name: &str,
    _typ: &Option<Type>,
    value: &Expr,
    ctx: &mut TranspileContext,
    is_mutable: bool,
) -> Option<Result<String, String>> {
    if let Some(args) = is_input_expr(value) {
        return Some(transpile_input_var(
            name,
            &Type::Metn,
            args,
            ctx,
            is_mutable,
        ));
    }

    if let Expr::MethodCall {
        target,
        method,
        args,
    } = value
    {
        if method == "böl" {
            let target_code = match transpile_expr(target, ctx) {
                Ok(code) => code,
                Err(e) => return Some(Err(e)),
            };

            let delimiter_code = match transpile_expr(&args[0], ctx) {
                Ok(code) => code.replace("\"", "'"),
                Err(e) => return Some(Err(e)),
            };

            let result_code = if is_mutable {
                ctx.needs_allocator = true;
                ctx.used_split_alloc_fn = true;
                ctx.cleanup_statements.push(format!("{name}.deinit();"));
                format!(
                    "var {name} = try splitNAlloc(allocator, {target}, {delim});",
                    name = name,
                    target = target_code,
                    delim = delimiter_code
                )
            } else {
                let result_var = format!("result_{}", name);
                ctx.used_split_n_fn = true;
                format!(
                    r#"const {result_var} = splitN({target}, {delim}, 32);
const {name} = {result_var}.parts[0..{result_var}.len];"#,
                    result_var = result_var,
                    target = target_code,
                    delim = delimiter_code,
                    name = name
                )
            };

            return Some(Ok(result_code));
        }
    }

    None
}
