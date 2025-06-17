use super::ast::{BuiltInFunction, Expr, Type};
use crate::transpiler::context::TranspileContext;
use crate::transpiler::utils::get_expr_type; // ctx tipini unutma

pub fn validate_expr(expr: &Expr, ctx: &mut TranspileContext) -> Result<(), String> {
    match expr {
        Expr::MutableDecl { name, typ, value } => {
            ctx.mutable_symbols.insert(name.clone());

            let inferred = get_expr_type(value, ctx)
                .ok_or_else(|| format!("'{}' üçün tip təyin edilə bilmədi", name))?;

            let declared = match typ {
                Some(t) => t.clone(),
                None => inferred.clone(), // tip göstərilməyibsə, infer et
            };

            if inferred != declared {
                return Err(format!(
                    "'{}' üçün tip uyğunsuzluğu: gözlənilən {:?}, tapılan {:?}",
                    name, declared, inferred
                ));
            }

            ctx.symbol_types.insert(name.clone(), declared);

            validate_expr(value, ctx)?;
        }
        Expr::ConstantDecl { name, typ, value } => {
            let inferred = get_expr_type(value, ctx)
                .ok_or_else(|| format!("'{}' üçün tip təyin edilə bilmədi", name))?;

            let declared = match typ {
                Some(t) => t.clone(),
                None => inferred.clone(), // tip göstərilməyibsə, infer et
            };

            if inferred != declared {
                return Err(format!(
                    "'{}' üçün tip uyğunsuzluğu: gözlənilən {:?}, tapılan {:?}",
                    name, declared, inferred
                ));
            }

            ctx.symbol_types.insert(name.clone(), declared);

            validate_expr(value, ctx)?;
        }

        Expr::If {
            condition,
            then_branch,
            ..
        } => {
            validate_expr(condition, ctx)?;

            let cond_type =
                get_expr_type(condition, ctx).ok_or("İf şərtinin tipi müəyyən edilə bilmədi")?;

            if cond_type != Type::Bool {
                return Err(format!(
                    "İf şərti `bool` olmalıdır, tapıldı: {:?}",
                    cond_type
                ));
            }

            for expr in then_branch {
                validate_expr(expr, ctx)?;
            }

            // Əgər else_branch əlavə olunubsa, onu da yoxla
            if let Expr::If {
                else_branch: Some(else_branch),
                ..
            } = expr
            {
                for expr in else_branch {
                    validate_expr(expr, ctx)?;
                }
            }
        }
        Expr::BinaryOp { left, op, right } => {
            validate_expr(left, ctx)?;
            validate_expr(right, ctx)?;

            let left_type = get_expr_type(left, ctx);
            let right_type = get_expr_type(right, ctx);

            if left_type != right_type {
                return Err(format!(
                    "Binary `{}` operatorunda tip uyğunsuzluğu: {:?} və {:?}",
                    op, left_type, right_type
                ));
            }
        }

        Expr::BuiltInCall { func, args } => {
            for arg in args {
                validate_expr(arg, ctx)?;
            }

            if *func == BuiltInFunction::Sum {
                if let Some(t) = get_expr_type(&args[0], ctx) {
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
            method,
            target,
            args,
        } => {
            validate_expr(target, ctx)?;
            for arg in args {
                validate_expr(arg, ctx)?;
            }
            validate_method_call(method, args)?;
        }

        Expr::FunctionCall { args, .. } => {
            for arg in args {
                validate_expr(arg, ctx)?;
            }
        }

        Expr::FunctionDef { body, .. } => {
            for expr in body {
                validate_expr(expr, ctx)?;
            }
        }

        Expr::Loop {
            iterable,
            var_name,
            body,
        } => {
            if let Some(Type::Siyahi(inner)) = get_expr_type(iterable, ctx) {
                // Dəyişəni kontekstə əlavə et
                ctx.symbol_types.insert(var_name.clone(), *inner);
            } else {
                println!("iterable type: {:?}", get_expr_type(iterable, ctx));
                println!("iterable: {:?}", iterable);
                return Err("Dövr üçün istifadə edilən obyekt siyahı olmalıdır".to_string());
            }

            for expr in body {
                validate_expr(expr, ctx)?;
            }
        }

        Expr::Return(e) => {
            validate_expr(e, ctx)?;
        }

        Expr::List(items) => {
            if items.is_empty() {
                return Ok(()); // boş siyahı üçün problem yoxdur
            }

            // İlk elementin tipi referans kimi götürülür
            let first_type = get_expr_type(&items[0], ctx)
                .ok_or("Siyahının ilk elementi üçün tip təyin edilə bilmədi")?;

            for item in items.iter().skip(1) {
                let t = get_expr_type(item, ctx)
                    .ok_or("Siyahı elementi üçün tip təyin edilə bilmədi")?;

                if t != first_type {
                    return Err(format!(
                        "Siyahı daxilində tip uyğunsuzluğu: {:?} və {:?}",
                        first_type, t
                    ));
                }
                validate_expr(item, ctx)?;
            }
        }
        Expr::Break => {}
        Expr::Continue => {}

        Expr::Index { target, index } => {
            validate_expr(target, ctx)?;
            validate_expr(index, ctx)?;
        }

        Expr::VariableRef(name) => {
            if !ctx.symbol_types.contains_key(name) {
                return Err(format!(
                    "Dəyişən '{}' istifadə olunmadan əvvəl elan edilməyib",
                    name
                ));
            }
        }

        // Terminal node-lar – yoxlamaya ehtiyac yoxdur
        Expr::String(_) | Expr::Bool(_) | Expr::Number(_) => {}
    }

    Ok(())
}

fn validate_method_call(method: &str, args: &[Expr]) -> Result<(), String> {
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

        _ => {
            return Err(format!("Dəstəklənməyən metod: {}", method));
        }
    }
    Ok(())
}
