use crate::{
    context::{Symbol, TranspileContext},
    expr::transpile_expr,
    parser::{
        Expr,
        ast::{BuiltInFunction, Type},
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
    typ: &Type,
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
    println!("Expr type: {:#?}", expr_type);
    if let Some(Type::Siyahi(_)) = expr_type {
        return Ok("".to_string());
    }
    let format_str = expr_type
        .map(|typ| get_format_str_from_type(&typ))
        .unwrap_or("{}");

    let mut arg_code = transpile_expr(expr, ctx)?;

    if let Expr::Index { target, .. } = expr {
        if let Expr::VariableRef(name) = &**target {
            if let Some(sym) = ctx.lookup_variable(name) {
                // mutable olub-olmamasına baxırıq
                if sym.is_mutable {
                    arg_code = arg_code.replace(name, &format!("{}.items", name));
                } else {
                    arg_code = arg_code.replace(name, &format!("{}", name));
                }
            }
        }
    }

    ctx.uses_stdout = true;
    Ok(format!(
        "std.debug.print(\"{}\\n\", .{{{}}});",
        format_str, arg_code
    ))
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

    // Tipi expr növündən asılı olaraq tapırıq, məsələn, sadə halda VariableRef üçün
    // ctx.lookup_variable-dən götürürük.
    let list_type = match list_expr {
        Expr::VariableRef(name) => ctx
            .lookup_variable(name)
            .map(|sym| sym.typ)
            .expect("Tip tapılmalıdır"),
        // Lazım gələrsə digər Expr növləri üçün də genişləndirilə bilər
        _ => panic!("sum() üçün list expr VariableRef olmalıdır"),
    };

    let inner_type = match list_type {
        Type::Siyahi(boxed) => boxed,
        _ => unreachable!("sum() yalnız siyahılar üçün"),
    };

    let type_code = match *inner_type {
        Type::Integer => "usize",
        Type::LowInteger => "u8",
        Type::BigInteger => "i128",
        _ => unreachable!("sum() yalnız rəqəm siyahıları üçün"),
    };

    ctx.used_sum_fn = true;
    if list_code.starts_with('[') && list_code.ends_with(']') {
        let stripped = &list_code[1..list_code.len() - 1];
        Ok(format!(
            "sum({}, &[_]{}{{ {} }})",
            type_code, type_code, stripped
        ))
    } else {
        Ok(format!("sum({}, {})", type_code, list_code))
    }
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
