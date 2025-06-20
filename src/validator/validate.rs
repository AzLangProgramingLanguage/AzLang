use crate::context::{Symbol, TranspileContext};
use crate::parser::Expr;
use crate::parser::ast::{BuiltInFunction, Type};
use crate::parser::types::get_type;

pub fn validate_expr(
    expr: &Expr,
    ctx: &mut TranspileContext,
    message: &mut dyn FnMut(&str),
) -> Result<(), String> {
    match expr {
        Expr::MutableDecl { name, typ, value } => {
            message(&format!("Dəyişən yaradılır: '{}'", name));

            let inferred = get_type(value, ctx)
                .ok_or_else(|| format!("'{}' üçün tip təyin edilə bilmədi", name))?;

            let declared = match typ {
                Some(t) => t.clone(),
                None => inferred.clone(),
            };

            if inferred != declared {
                return Err(format!(
                    "{} üçün tip uyğunsuzluğu: gözlənilən {:?}, tapılan {:?}",
                    name, declared, inferred
                ));
            }

            ctx.symbol_types.insert(
                name.clone(),
                Symbol {
                    typ: declared,
                    is_mutable: true,
                    is_used: false,
                    is_param: false,
                    source_location: None,
                },
            );

            validate_expr(value, ctx, message)?;
        }

        Expr::StructDef {
            name,
            fields,
            methods,
        } => {
            message(&format!("Struktur elan edilir: '{}'", name));

            // Eyni adda struktur varsa, xəta qaytar
            if ctx.struct_defs.contains_key(name) {
                return Err(format!("Struktur '{}' artıq mövcuddur", name));
            }

            // Sadəcə sahələri yadda saxlayırıq — metodları ayrıca saxlamaq istəmirsənsə
            ctx.struct_defs
                .insert(name.clone(), (fields.clone(), methods.clone()));
        }

        Expr::StructInit { name, args } => {
            message(&format!("Struktur yaradılır: '{}'", name));

            let (struct_fields, _methods) = ctx
                .struct_defs
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Struktur '{}' tapılmadı", name))?;

            if struct_fields.len() != args.len() {
                return Err(format!(
                    "'{}' strukturu üçün {} arqument gözlənilirdi, lakin {} verildi",
                    name,
                    struct_fields.len(),
                    args.len()
                ));
            }

            for ((field_name, expected_type), arg_expr) in struct_fields.iter().zip(args.iter()) {
                let actual_type = get_type(arg_expr, ctx).ok_or_else(|| {
                    format!("'{}' sahəsi üçün tip təyin edilə bilmədi", field_name)
                })?;

                validate_expr(arg_expr, ctx, message)?;

                if &actual_type != expected_type {
                    return Err(format!(
                        "'{}' sahəsi üçün tip uyğunsuzluğu: gözlənilən {:?}, tapılan {:?}",
                        field_name, expected_type, actual_type
                    ));
                }
            }
        }

        Expr::FieldAccess { target, field } => {
            validate_expr(target, ctx, message)?;

            let target_type = get_type(target, ctx)
                .ok_or_else(|| "FieldAccess üçün tip təyin edilə bilmədi".to_string())?;

            let struct_name = if let Type::Istifadeci(name) = target_type {
                name
            } else {
                return Err("Sahəyə yalnız struktur növü üzərindən çıxış edilə bilər".to_string());
            };

            let (struct_fields, _methods) = ctx
                .struct_defs
                .get(&struct_name)
                .ok_or_else(|| format!("Struktur '{}' tapılmadı", struct_name))?;

            let found = struct_fields.iter().any(|(f, _)| f == field);

            if !found {
                return Err(format!(
                    "'{}' strukturu sahəyə sahib deyil: '{}'",
                    struct_name, field
                ));
            }
        }

        Expr::ConstantDecl { name, typ, value } => {
            message(&format!("Sabit yaradılır: '{}'", name));

            let inferred = get_type(value, ctx)
                .ok_or_else(|| format!("'{}' üçün tip təyin edilə bilmədi", name))?;

            let declared = match typ {
                Some(t) => t.clone(),
                None => inferred.clone(),
            };

            if inferred != declared {
                return Err(format!(
                    "{} üçün tip uyğunsuzluğu: gözlənilən {:?}, tapılan {:?}",
                    name, declared, inferred
                ));
            }

            ctx.symbol_types.insert(
                name.clone(),
                Symbol {
                    typ: declared,
                    is_mutable: false,
                    is_used: false,
                    is_param: false,
                    source_location: None,
                },
            );

            validate_expr(value, ctx, message)?;
        }

        Expr::Assignment { name, value } => {
            message(&format!("Mənimsətmə yoxlanılır: {} = ...", name));

            // Simvolun olub-olmadığını yoxla
            let symbol = ctx
                .lookup_variable(name)
                .ok_or_else(|| format!("Dəyişən '{}' elan edilməyib", name))?;

            // Əgər mutable deyilsə, xəta qaytar
            if !symbol.is_mutable {
                return Err(format!("Sabit '{}' dəyişdirilə bilməz", name));
            }

            // Tip uyğunsuzluğu varsa xəta qaytar
            let value_type = get_type(value, ctx)
                .ok_or_else(|| format!("{} üçün tip təyin edilə bilmədi", name))?;

            if value_type != symbol.typ {
                return Err(format!(
                    "Tip uyğunsuzluğu: '{}' üçün {:?} gözlənilirdi, lakin {:?} tapıldı",
                    name, symbol.typ, value_type
                ));
            }

            validate_expr(value, ctx, message)?; // Dəyərin özünü də validasiya et
        }

        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            message("Şərtə baxılır (if)");

            validate_expr(condition, ctx, message)?;

            let cond_type =
                get_type(condition, ctx).ok_or("İf şərtinin tipi müəyyən edilə bilmədi")?;

            if cond_type != Type::Bool {
                return Err(format!(
                    "İf şərti `bool` olmalıdır, tapıldı: {:?}",
                    cond_type
                ));
            }

            for expr in then_branch {
                validate_expr(expr, ctx, message)?;
            }

            if let Some(else_branch) = else_branch {
                message("Else hissəsi də yoxlanılır");
                for expr in else_branch {
                    validate_expr(expr, ctx, message)?;
                }
            }
        }

        Expr::BinaryOp { left, op, right } => {
            message(&format!("İki tərəfli əməliyyat: {:?}", op));

            validate_expr(left, ctx, message)?;
            validate_expr(right, ctx, message)?;

            let left_type = get_type(left, ctx);
            let right_type = get_type(right, ctx);

            if left_type != right_type {
                return Err(format!(
                    "Binary `{}` operatorunda tip uyğunsuzluğu: {:?} və {:?}",
                    op, left_type, right_type
                ));
            }
        }

        Expr::BuiltInCall {
            func,
            args,
            resolved_type: _,
        } => {
            message(&format!("Daxili funksiya çağırılır: {:?}", func));

            for arg in args {
                validate_expr(arg, ctx, message)?;
            }

            if *func == BuiltInFunction::Sum {
                if let Some(t) = get_type(&args[0], ctx) {
                    match t {
                        Type::Siyahi(inner) if *inner == Type::Integer => {}
                        _ => {
                            return Err(
                                "sum funksiyası yalnız ədəd tipli siyahı qəbul edir".to_string()
                            );
                        }
                    }
                }
            }
        }

        Expr::MethodCall {
            target,
            method,
            args,
        } => {
            validate_expr(target, ctx, message)?;
            for arg in args {
                validate_expr(arg, ctx, message)?;
            }

            let target_type = get_type(target, ctx)
                .ok_or_else(|| "MethodCall üçün tip təyin edilə bilmədi".to_string())?;

            validate_method_call(&target_type, method, args, ctx)?;
        }

        Expr::FunctionCall { args, .. } => {
            message("Funksiya çağırışı yoxlanılır");

            for arg in args {
                validate_expr(arg, ctx, message)?;
            }
        }
        Expr::FunctionDef {
            name, params, body, ..
        } => {
            message(&format!("Funksiya tərifi: {}", name));

            ctx.push_scope();

            for param in params {
                message(&format!(
                    "Parametr əlavə edilir: {}: {:?}",
                    param.name, param.typ
                ));

                let symbol = Symbol {
                    typ: param.typ.clone(),
                    is_mutable: param.is_mutable,
                    is_used: false,
                    is_param: true,
                    source_location: None,
                };

                ctx.declare_variable(param.name.clone(), symbol);
            }

            for stmt in body {
                validate_expr(stmt, ctx, message)?;
            }

            ctx.pop_scope();
            return Ok(());
        }

        Expr::Loop {
            iterable,
            var_name,
            body,
        } => {
            message("Dəmir Əmi dövrü yoxlayır...");

            validate_expr(iterable, ctx, message)?;

            let iterable_type = get_type(iterable, ctx).ok_or_else(|| {
                "Dövr üçün istifadə edilən obyektin tipi təyin edilə bilmədi".to_string()
            })?;

            if let Type::Siyahi(inner) = iterable_type {
                let symbol = Symbol {
                    typ: *inner,
                    is_mutable: false,
                    is_used: false,
                    is_param: false,
                    source_location: None,
                };

                ctx.declare_variable(var_name.clone(), symbol);
                message(&format!(
                    "Dəyişən `{}` siyahıdan götürülərək elan edildi",
                    var_name
                ));
            } else {
                return Err(
                    "Dövr üçün istifadə edilən obyekt siyahı (`list`) olmalıdır".to_string()
                );
            }

            for expr in body {
                validate_expr(expr, ctx, message)?;
            }
        }

        Expr::Return(expr) => {
            message("Dəmir Əmi return ifadəsini yoxlayır...");
            validate_expr(expr, ctx, message)?;
        }

        Expr::List(items) => {
            message("Dəmir Əmi siyahını yoxlayır...");

            if items.is_empty() {
                message("Boş siyahı tapıldı, problem yoxdur.");
                return Ok(()); // boş siyahı üçün problem yoxdur
            }

            let first_type = get_type(&items[0], ctx).ok_or_else(|| {
                let msg = "Siyahının ilk elementi üçün tip təyin edilə bilmədi";
                message(msg);
                msg.to_string()
            })?;

            for item in items.iter().skip(1) {
                let t = get_type(item, ctx).ok_or_else(|| {
                    let msg = "Siyahı elementi üçün tip təyin edilə bilmədi";
                    message(msg);
                    msg.to_string()
                })?;

                if t != first_type {
                    let msg = format!(
                        "Siyahı daxilində tip uyğunsuzluğu: {:?} və {:?}",
                        first_type, t
                    );
                    message(&msg);
                    return Err(msg);
                }

                validate_expr(item, ctx, message)?;
            }
        }
        Expr::Break => {}
        Expr::Continue => {}

        Expr::Index { target, index } => {
            message("Dəmir Əmi indeksləmə əməliyyatını yoxlayır...");
            validate_expr(target, ctx, message)?;
            validate_expr(index, ctx, message)?;
        }

        Expr::VariableRef(name) => {
            message(&format!("Dəmir Əmi dəyişənə baxır: `{}`", name));
            if ctx.lookup_variable(name).is_none() {
                let msg = format!("Dəyişən '{}' istifadə olunmadan əvvəl elan edilməyib", name);
                message(&msg);
                return Err(msg);
            }
        }

        Expr::String(_) | Expr::Bool(_) | Expr::Number(_) => {}
    }

    Ok(())
}

fn validate_method_call(
    target_type: &Type,
    method: &str,
    args: &[Expr],
    ctx: &TranspileContext,
) -> Result<(), String> {
    match target_type {
        Type::Metn | Type::Siyahi(_) => {
            // Built-in metodların yoxlanması
            match method {
                "əlavə_et" | "sil" | "sıralı_sil" => {
                    if args.len() != 1 {
                        return Err(format!("{} metodu yalnız 1 arqument qəbul edir", method));
                    }
                }

                "sırala" | "əks_sırala" | "uzunluq" | "boşdur" => {
                    if !args.is_empty() {
                        return Err(format!("{} metodu arqumentsiz olmalıdır", method));
                    }
                }

                "cəm" | "sum" => {
                    if args.len() != 1 {
                        return Err(format!("{} metodu yalnız 1 arqument qəbul edir", method));
                    }
                }

                "aralıq" | "range" => {
                    if args.len() != 2 {
                        return Err(format!("{} metodu yalnız 2 arqument qəbul edir", method));
                    }
                }

                "böyüt" | "kiçilt" | "kənar_təmizlə" => {
                    if !args.is_empty() {
                        return Err(format!("{} metodu arqumentsiz olmalıdır", method));
                    }
                }

                "əvəzlə" | "kəs" => {
                    if args.len() != 2 {
                        return Err(format!("{} metodu 2 arqument qəbul edir", method));
                    }
                }

                "birləşdir" | "böl" => {
                    if args.len() != 1 {
                        return Err(format!("{} metodu yalnız 1 arqument qəbul edir", method));
                    }
                }

                _ => return Err(format!("Dəstəklənməyən metod: {}", method)),
            }

            Ok(())
        }

        Type::Istifadeci(struct_name) => {
            // İstifadəçi tərəfindən təyin olunan metodları tap
            let (_, methods) = ctx
                .struct_defs
                .get(struct_name)
                .ok_or_else(|| format!("Struktur tapılmadı: {}", struct_name))?;

            for (method_name, params, _body, _ret_type) in methods {
                if method_name == method {
                    // `self` avtomatik verilir, ona görə 1 çıxılır
                    let expected_arg_count = params.iter().filter(|p| p.name != "self").count();
                    if args.len() != expected_arg_count {
                        return Err(format!(
                            "'{}' metodu {} arqument qəbul edir, amma {} verildi",
                            method,
                            expected_arg_count,
                            args.len()
                        ));
                    } else {
                        return Ok(());
                    }
                }
            }

            Err(format!(
                "'{}' strukturu belə bir metoda sahib deyil: '{}'",
                struct_name, method
            ))
        }

        _ => Err(format!("Tip metodları dəstəkləmir: {:?}", target_type)),
    }
}
