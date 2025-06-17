use crate::{
    builtin::get_format_str_from_type,
    context::TranspileContext,
    expr::transpile_expr,
    parser::{
        Expr,
        ast::{BuiltInFunction, Type},
    },
};

pub fn map_type(typ: &Type, is_const: bool) -> String {
    match typ {
        Type::Integer => "usize".to_string(), // Zig-də təxminən: unsigned native integer
        Type::Any => "any".to_string(),
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
                format!("const {}", name)
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
    if args.len() != 1 {
        return Err("input funksiyası yalnız 1 arqument qəbul edir".to_string());
    }
    ctx.symbol_types.insert(name.to_string(), Type::Metn);

    if let Type::Metn = typ {
        let prompt = transpile_expr(&args[0], ctx)?;
        let buf_name = format!("buf_{}", name);
        ctx.used_input_fn = true;
        if is_mutable {
            Ok(format!(
                "var {buf}: [100]u8 = undefined;\nvar {var}: []u8 = try input({prompt}, &{buf});",
                buf = buf_name,
                var = name,
                prompt = prompt
            ))
        } else {
            Ok(format!(
                "var {buf}: [100]u8 = undefined;\nconst {var}: []u8 = try input({prompt}, &{buf});",
                buf = buf_name,
                var = name,
                prompt = prompt
            ))
        }
    } else {
        Err("input funksiyası yalnız metn tipli dəyişənlərə tətbiq oluna bilər.".to_string())
    }
}

pub fn transpile_builtin_print(expr: &Expr, ctx: &mut TranspileContext) -> Result<String, String> {
    let expr_type = get_expr_type(expr, ctx);

    if let Some(Type::Siyahi(_)) = expr_type {
        return Err(
            "çap() ilə siyahı çıxarda bilməzsiniz, zəhmət olmasa yazdır() istifadə edin"
                .to_string(),
        );
    }

    if let Some(tip) = expr_type {
        let format_str = get_format_str_from_type(&tip);
        let mut arg_code = transpile_expr(expr, ctx)?;

        // 🧠 Əgər ifadə bir indekslənmiş siyahıdırsa, items əlavə et
        if let Expr::Index { target, .. } = expr {
            if let Expr::VariableRef(name) = &**target {
                if let Some(Type::Siyahi(_)) = ctx.symbol_types.get(name) {
                    // ededler -> ededler.items
                    if ctx.mutable_symbols.contains(name) {
                        arg_code = arg_code.replace(name, &format!("{}.items", name));
                    } else {
                        arg_code = arg_code.replace(name, &format!("{}", name));
                    }
                }
            }
        }

        Ok(format!(
            "std.debug.print(\"{}\\n\", .{{{}}});",
            format_str, arg_code
        ))
    } else {
        Err("Tipi müəyyən etmək mümkün olmadı".to_string())
    }
}

pub fn get_expr_type(expr: &Expr, ctx: &TranspileContext) -> Option<Type> {
    match expr {
        Expr::Index { target, .. } => {
            let target_type = get_expr_type(target, ctx)?;

            match target_type {
                Type::Siyahi(inner) => Some(*inner),
                Type::Metn => Some(Type::Char),
                _ => None,
            }
        }
        Expr::List(items) => {
            if items.is_empty() {
                return Some(Type::Siyahi(Box::new(Type::Any))); // boş siyahı – tipi bilinmir
            }

            let item_type = get_expr_type(&items[0], ctx)?;

            for item in &items[1..] {
                let t = get_expr_type(item, ctx)?;
                if t != item_type {
                    return Some(Type::Siyahi(Box::new(Type::Any))); // qarışıq tiplər
                }
            }

            Some(Type::Siyahi(Box::new(item_type)))
        }

        Expr::VariableRef(name) => ctx.symbol_types.get(name).cloned(),
        Expr::String(_) => Some(Type::Metn),
        Expr::Number(_) => Some(Type::Integer),
        Expr::Bool(_) => Some(Type::Bool),

        Expr::MethodCall { target, method, .. } => {
            let target_type = get_expr_type(target, ctx);
            match target_type {
                Some(Type::Siyahi(_)) => match method.as_str() {
                    "uzunluq" | "boşdur" => Some(Type::Integer),
                    _ => None,
                },
                Some(Type::Metn) => match method.as_str() {
                    "uzunluq" | "boşdur" => Some(Type::Integer),
                    "böyüt" | "kiçilt" | "kənar_təmizlə" => Some(Type::Metn),
                    _ => None,
                },
                _ => None,
            }
        }

        Expr::FunctionCall { name, .. } => match name.as_str() {
            "print" => Some(Type::Metn),
            "input" => Some(Type::Metn),
            "number" => Some(Type::Integer),
            _ => None,
        },
        Expr::BuiltInCall { func, args } => match func {
            BuiltInFunction::Print => Some(Type::Metn),
            BuiltInFunction::Len | BuiltInFunction::Number | BuiltInFunction::Sum => {
                Some(Type::Integer)
            }
            BuiltInFunction::LastWord => Some(Type::Metn),
            BuiltInFunction::Input => Some(Type::Metn),
            BuiltInFunction::Range => {
                if args.len() == 2 {
                    let left = get_expr_type(&args[0], ctx)?;
                    let right = get_expr_type(&args[1], ctx)?;

                    if left == Type::Integer && right == Type::Integer {
                        Some(Type::Siyahi(Box::new(Type::Integer)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        },

        Expr::BinaryOp { left, op, right } => {
            let left_type = get_expr_type(left, ctx)?;
            let right_type = get_expr_type(right, ctx)?;

            if left_type != right_type {
                return None;
            }

            // Müqayisə operatorları üçün nəticə həmişə `Bool` olur
            let comparison_ops = ["==", "!=", "<", "<=", ">", ">="];
            let logic_ops = ["&&", "||"];
            if comparison_ops.contains(&op.as_str()) || logic_ops.contains(&op.as_str()) {
                return Some(Type::Bool);
            }

            // Əks halda arifmetik və ya digər operatorlardır – nəticə operandların tipidir
            Some(left_type)
        }

        _ => None,
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

    let list_type = get_expr_type(list_expr, ctx).ok_or("sum() üçün siyahının tipi tapılmadı")?;
    let inner_type = match list_type {
        Type::Siyahi(boxed) => boxed,
        _ => return Err("sum() yalnız siyahılar üçün keçərlidir".to_string()),
    };

    let type_code = match *inner_type {
        Type::Integer => "usize",
        Type::LowInteger => "u8",
        Type::BigInteger => "i128",
        _ => return Err("sum() yalnız rəqəm siyahıları üçün keçərlidir".to_string()),
    };

    ctx.used_sum_fn = true;
    if list_code.starts_with("[") && list_code.ends_with("]") {
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
