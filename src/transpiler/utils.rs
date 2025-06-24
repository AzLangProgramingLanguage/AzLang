use crate::{
    context::{Symbol, TranspileContext},
    expr::transpile_expr,
    parser::{
        Expr,
        ast::{TemplateChunk, Type},
        types::get_type,
    },
};

pub fn map_type(typ: &Type, is_const: bool) -> String {
    match typ {
        Type::Integer => "usize".to_string(), // Zig-də təxminən: unsigned native integer
        Type::Any => "any".to_string(),
        Type::Void => "void".to_string(),
        Type::BigInteger => {
            if is_const {
                "const i128".to_string()
            } else {
                "i128".to_string()
            }
        }
        Type::Char => "u8".to_string(),
        Type::LowInteger => {
            if is_const {
                "const u8".to_string()
            } else {
                "u8".to_string()
            }
        }

        Type::Metn => {
            if is_const {
                "[]const u8".to_string()
            } else {
                "[]u8".to_string()
            }
        }

        Type::Bool => "bool".to_string(),
        Type::Siyahi(inner) => {
            let inner_str = map_type(inner, is_const);
            if is_const {
                format!("[]const {}", inner_str)
            } else {
                format!("[]{}", inner_str)
            }
        }

        Type::Istifadeci(name) => {
            if is_const {
                format!("{}", name)
            } else {
                name.clone()
            }
        }
    }
}

pub fn transpile_input_var(
    name: &str,
    _typ: &Type,
    args: &[Expr],
    ctx: &mut TranspileContext,
    is_mutable: bool,
) -> Result<String, String> {
    // Validator buna görə artıq yoxlama aparmayacaq, ona görə sadəcə icra.

    // Context-ə dəyişəni qeyd et (mutable və digər sahələri default qoyulur)
    ctx.declare_variable(
        name.to_string(),
        Symbol {
            typ: Type::Metn,
            is_mutable,
            is_used: false,
            is_param: false,
            source_location: None,
        },
    );

    // Prompt kodunu transpile et
    let prompt = transpile_expr(&args[0], ctx)?;

    // Input funksiyasının çağırışı üçün buffer və dəyişən təyin et
    let buf_name = format!("buf_{}", name);
    ctx.used_input_fn = true;

    let var_decl = if is_mutable {
        format!(
            "var {buf}: [100]u8 = undefined;\nvar {var}: []u8 = try input({prompt}, &{buf});",
            buf = buf_name,
            var = name,
            prompt = prompt
        )
    } else {
        format!(
            "var {buf}: [100]u8 = undefined;\nconst {var}: []u8 = try input({prompt}, &{buf});",
            buf = buf_name,
            var = name,
            prompt = prompt
        )
    };

    Ok(var_decl)
}

pub fn transpile_builtin_print(expr: &Expr, ctx: &mut TranspileContext) -> Result<String, String> {
    let expr_type = get_type(expr, ctx);
    let format_str = expr_type
        .map(|typ| get_format_str_from_type(&typ))
        .unwrap_or("{}");

    match expr {
        Expr::TemplateString(chunks) => {
            let mut format_parts = String::new();
            let mut args = Vec::new();

            for chunk in chunks {
                match chunk {
                    TemplateChunk::Literal(s) => {
                        format_parts.push_str(&s.replace("\\", "\\\\").replace("\"", "\\\""));
                    }
                    TemplateChunk::Expr(inner_expr) => {
                        let expr_type = get_type(inner_expr, ctx);
                        let format_str = expr_type
                            .map(|typ| get_format_str_from_type(&typ))
                            .unwrap_or("{}");

                        format_parts.push_str(format_str);

                        let arg_code = match &**inner_expr {
                            Expr::VariableRef(name) => {
                                if let Some((_lvl, sym)) = ctx.lookup_variable_scoped(name) {
                                    if sym.is_param && sym.is_mutable {
                                        format!("{}.*", name)
                                    } else {
                                        name.clone()
                                    }
                                } else {
                                    transpile_expr(inner_expr, ctx)?
                                }
                            }
                            _ => transpile_expr(inner_expr, ctx)?,
                        };

                        args.push(arg_code);
                    }
                }
            }

            let args_code = if args.is_empty() {
                "".to_string()
            } else {
                format!(", .{{ {} }}", args.join(", "))
            };

            ctx.uses_stdout = true;
            Ok(format!(
                "std.debug.print(\"{}\\n\"{})",
                format_parts, args_code
            ))
        }

        _ => {
            let arg_code = match &*expr {
                Expr::VariableRef(name) => {
                    if let Some((_lvl, sym)) = ctx.lookup_variable_scoped(name) {
                        if sym.is_param && sym.is_mutable {
                            format!("{}.*", name)
                        } else {
                            name.clone()
                        }
                    } else {
                        transpile_expr(expr, ctx)?
                    }
                }
                _ => transpile_expr(expr, ctx)?,
            };

            ctx.uses_stdout = true;
            Ok(format!(
                "std.debug.print(\"{}\\n\", .{{{}}})",
                format_str, arg_code
            ))
        }
    }
}

//Todo burada typ.as_ref().unwrap() yazılır
pub fn is_mutable_decl(expr: &Expr) -> Option<(&str, &Type)> {
    match expr {
        Expr::MutableDecl { name, typ, .. } => Some((name.as_str(), typ.as_ref().unwrap())),
        _ => None,
    }
}

pub fn transpile_builtin_sum(args: &[Expr], ctx: &mut TranspileContext) -> Result<String, String> {
    let list_expr = &args[0];
    let list_code = transpile_expr(list_expr, ctx)?;

    // Siyahının tipini təyin et
    let inner_type = match list_expr {
        Expr::VariableRef(name) => {
            let (_, symbol) = ctx
                .lookup_variable_scoped(name)
                .ok_or("Dəyişən tapılmadı")?;

            match symbol.typ {
                Type::Siyahi(ref boxed) => boxed.clone(),
                _ => return Err("sum() yalnız siyahılar üçün keçərlidir".to_string()),
            }
        }
        _ => {
            let list_type = get_type(list_expr, ctx).ok_or("sum() üçün tip təyin edilə bilmədi")?;

            match list_type {
                Type::Siyahi(boxed) => boxed,
                _ => return Err("sum() yalnız siyahılar üçün keçərlidir".to_string()),
            }
        }
    };

    // Tip kodunu müəyyən et
    let type_code = match *inner_type {
        Type::Integer => "usize",
        Type::LowInteger => "u8",
        Type::BigInteger => "i128",
        _ => return Err("sum() yalnız rəqəm siyahıları üçün işləyir".to_string()),
    };

    ctx.used_sum_fn = true;

    // Mutable siyahı isə `.items` əlavə et
    let final_list_code = match list_expr {
        Expr::VariableRef(name) => {
            let (_, symbol) = ctx.lookup_variable_scoped(name).unwrap(); // Artıq yoxlanılıb
            if symbol.is_mutable {
                format!("{}.items", name)
            } else {
                name.clone()
            }
        }
        _ => {
            if list_code.starts_with('[') && list_code.ends_with(']') {
                let stripped = &list_code[1..list_code.len() - 1];
                format!("&[_]{}{{ {} }}", type_code, stripped)
            } else {
                list_code.clone()
            }
        }
    };

    Ok(format!("sum({}, {})", type_code, final_list_code))
}

pub fn transpile_builtin_range(
    args: &[Expr],
    ctx: &mut TranspileContext,
) -> Result<String, String> {
    if args.len() != 2 {
        return Err("range funksiyası 2 arqument qəbul etməlidir".to_string());
    }

    let start_code = transpile_expr(&args[0], ctx)?;
    let end_code = transpile_expr(&args[1], ctx)?;

    // Zig sintaksisi: `start..end`
    Ok(format!("{}..{}", start_code, end_code))
}

pub fn get_format_str_from_type(typ: &Type) -> &'static str {
    match typ {
        Type::Metn => "{s}",
        Type::Integer | Type::BigInteger | Type::LowInteger => "{}",
        Type::Bool => "{}",
        Type::Char => "{c}",
        Type::Void => "",
        Type::Any => "{any}",
        Type::Siyahi(_) => "{any} ", // Siyahıları yazdırmaq istəmirik, amma fallback
        Type::Istifadeci(_) => "{any}", // Custom tip varsa default yazdırma
    }
}
