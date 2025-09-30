use std::{borrow::Cow, rc::Rc};

use color_eyre::eyre::Result;

use crate::{
    dd,
    parser::ast::{BuiltInFunction, EnumDecl, Expr, Symbol, TemplateChunk, Type},
    translations::validator_messages::ValidatorError,
    validator::{FunctionInfo, MethodInfo, ValidatorContext, helpers::get_type},
};

pub fn validate_expr<'a>(
    expr: &mut Expr<'a>,
    ctx: &mut ValidatorContext<'a>,
    log: &mut dyn FnMut(&str),
) -> Result<(), ValidatorError<'a>> {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            log(&format!("✅ Declarasiya yaradılır: {name}"));
            log(&format!(
                "{} yaradılır: '{}'",
                if *is_mutable { "Dəyişən" } else { "Sabit" },
                name
            ));
            if ctx.lookup_variable(name).is_some() {
                return Err(ValidatorError::AlreadyDecl(name.to_string()));
            }

            validate_expr(value, ctx, log)?;
            let inferred = get_type(value, ctx, typ.as_deref());
            if let Some(s) = inferred {
                if let Some(typ_ref) = typ.as_deref() {
                    if *typ_ref != s {
                        return Err(ValidatorError::DeclTypeMismatch {
                            name: name.to_string(),
                            expected: format!("{s:?}"),
                            found: format!("{typ_ref:?}"),
                        });
                    }
                }
                *typ = Some(Rc::new(s.clone()));
                ctx.declare_variable(
                    name.to_string(),
                    Symbol {
                        typ: s,
                        is_mutable: *is_mutable,
                        is_used: false,
                        is_pointer: false,
                    },
                );
            } else {
                return Err(ValidatorError::DeclTypeUnknown);
            }
        }
        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            log(&format!("✅ Assignment yoxlanılır: '{name}'"));
            validate_expr(value, ctx, log)?;
            let inferred = get_type(value, ctx, None);
            if let Some(mut var) = ctx.lookup_variable(name) {
                if let Some(s) = inferred {
                    if var.typ != s {
                        return Err(ValidatorError::AssignmentTypeMismatch {
                            name: name.to_string(),
                            expected: format!("{s:?}"),
                            found: format!("{:?}", var.typ),
                        });
                    }
                    var.is_used = true;
                    if !var.is_mutable {
                        return Err(ValidatorError::AssignmentToImmutableVariable(
                            name.to_string(),
                        ));
                    }
                }
            } else {
                return Err(ValidatorError::UndefinedVariable(name.to_string()));
            }
        }
        Expr::UnionType {
            name,
            fields,
            methods,
        } => {
            log(&format!("✅ Union tərifi yoxlanılır: '{name}'"));
            if ctx.union_defs.contains_key(*name) {
                return Err(ValidatorError::DuplicateUnion(name.to_string()));
            }
            ctx.union_defs
                .insert(name.to_string(), (Vec::new(), Vec::new()));
            let method_infos: Vec<MethodInfo<'a>> = methods
                .iter()
                .map(|method| {
                    Ok(MethodInfo {
                        name: Cow::Borrowed(method.name),
                        return_type: method.return_type.clone(),
                        parameters: method.params.clone(),
                        is_allocator_used: false,
                    })
                })
                .collect::<Result<_, ValidatorError<'a>>>()?;

            let newfields: Vec<(&str, Type)> = fields
                .iter()
                .map(|(name, typ)| (*name, typ.clone()))
                .collect();
            ctx.union_defs
                .insert(name.to_string(), (newfields, method_infos));
            for method in methods.iter_mut() {
                ctx.current_struct = Some(name);
                for expr in &mut method.body {
                    validate_expr(expr, ctx, log)?;
                }

                if let Some(Type::Istifadeci(name)) = &mut method.return_type {
                    match ctx.validate_user_type(name.as_ref()) {
                        Ok(_) => {}
                        Err(e) => return Err(e),
                    }
                }
                if let Some((_fields, ctx_methods)) = ctx.union_defs.get_mut(*name) {
                    if let Some(ctx_method) = ctx_methods.iter_mut().find(|m| m.name == method.name)
                    {
                        ctx_method.is_allocator_used = ctx.is_allocator_used;
                    }
                }
                ctx.is_allocator_used = false;
            }

            ctx.current_struct = None;
        }
        Expr::Match { target, arms } => {
            log(&format!("✅ Match ifadəsi yoxlanılır"));
            validate_expr(target, ctx, log)?;
            for arm in arms {
                for expr in arm.1.iter_mut() {
                    validate_expr(expr, ctx, log)?;
                }
            }
        }
        Expr::String(_, _)
        | Expr::Float(_)
        | Expr::Bool(_)
        | Expr::Number(_)
        | Expr::UnaryOp { .. } => {}
        Expr::BuiltInCall {
            function,
            args,
            return_type: _,
        } => {
            log(&format!("✅ Built-in funksiya yoxlanılır: {function:?}"));

            if function.expected_arg_count().is_some() {
                let expected = function.expected_arg_count().unwrap();
                if args.len() != expected {
                    return Err(ValidatorError::InvalidArgumentCount {
                        name: function.to_string(),
                        expected,
                        found: args.len(),
                    });
                }
            }
            match function {
                BuiltInFunction::Allocator | BuiltInFunction::Trim => {
                    ctx.is_allocator_used = true;
                }
                BuiltInFunction::Print => {
                    /* TODO: Burada Void geçmesine icaze verme */
                    validate_expr(&mut args[0], ctx, log)?;
                    log(&format!("✅ Print funksiyası yoxlanılır"));
                    if let Some(t) = get_type(&args[0], ctx, None) {
                        if t == Type::Void {
                            return Err(ValidatorError::TypeMismatch {
                                expected: "Yazı".to_string(),
                                found: format!("{t:?}"),
                            });
                        }
                    }
                }
                BuiltInFunction::StrLower => {
                    log(&format!("✅ StrLower funksiyası yoxlanılır"));
                    ctx.is_allocator_used = true;
                }
                BuiltInFunction::StrReverse => {
                    log(&format!("✅ StrReverse funksiyası yoxlanılır"));
                    ctx.is_allocator_used = true;
                }
                BuiltInFunction::ConvertString => {
                    log(&format!("✅ ConvertString funksiyası yoxlanılır"));
                    ctx.is_allocator_used = true;
                }

                BuiltInFunction::StrUpper => {
                    log(&format!("✅ StrUpper funksiyası yoxlanılır"));
                    ctx.is_allocator_used = true;
                    /* TODO burada içeriden yoxlamanı et */
                    /*    if let Some(t) = get_type(&args[0], ctx, None) {
                        if t != Type::Metn {
                            return Err(ValidatorError::TypeMismatch {
                                expected: "Metn".to_string(),
                                found: format!("{t:?}"),
                            });
                        }
                    }
                    if args.len() != 1 {
                        return Err(ValidatorError::InvalidOneArgumentCount {
                            name: "StrUpper".to_string(),
                        });
                    } */
                }
                BuiltInFunction::Len => {
                    if let Some(t) = get_type(&args[0], ctx, None) {
                        if t != Type::Siyahi(Box::new(Type::Any)) {
                            return Err(ValidatorError::TypeMismatch {
                                expected: "Siyahi".to_string(),
                                found: format!("{t:?}"),
                            });
                        }
                    }
                    if args.len() != 1 {
                        return Err(ValidatorError::InvalidOneArgumentCount {
                            name: "Uzunluq".to_string(),
                        });
                    }
                }
                _ => {}
            }
            for arg in args {
                validate_expr(arg, ctx, log)?;
            }
        }
        Expr::StructInit {
            name,
            transpiled_name,
            args,
        } => {
            log(&format!("✅ Struct yoxlanılır: '{}'", name));

            if let Some((s, ..)) = ctx.struct_defs.get(name.as_ref()) {
            } else if let Some((s, ..)) = ctx.union_defs.get(name.as_ref()) {
            } else {
                return Err(ValidatorError::UnknownStruct(name.to_string()));
            }

            for arg in args.iter_mut() {
                validate_expr(&mut arg.1, ctx, log)?;
            }
        }
        Expr::StructDef {
            name,
            fields,
            methods,
        } => {
            log(&format!("✅ Struct tərifi yoxlanılır: '{}'", name));
            if ctx.struct_defs.contains_key(*name) {
                return Err(ValidatorError::DuplicateStruct(name));
            }

            let method_infos = methods
                .iter()
                .map(|method| {
                    let mut cloned_ret_type = method.return_type.clone();
                    if let Some(Type::Istifadeci(name)) = &mut cloned_ret_type {
                        match ctx.validate_user_type(name.as_ref()) {
                            Ok(_) => {}
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(MethodInfo {
                        name: Cow::Borrowed(method.name),
                        return_type: cloned_ret_type,
                        parameters: method.params.clone(),
                        is_allocator_used: false, // bu sonra müəyyən olunacaq
                    })
                })
                .collect::<Result<Vec<_>, ValidatorError<'a>>>()?;

            let newfields: Vec<(&str, Type)> = fields
                .iter()
                .map(|(name, typ, _)| (*name, typ.clone()))
                .collect();
            ctx.struct_defs
                .insert(name.to_string(), (newfields, method_infos));
            for method in methods.iter_mut() {
                ctx.current_struct = Some(name);
                for expr in &mut method.body {
                    validate_expr(expr, ctx, log)?;
                }
                log(&format!(
                    "✅ Struct metodları yoxlanılır: '{}'",
                    method.name
                ));

                ctx.is_allocator_used = false;
                ctx.current_struct = None;
            }
            std::mem::take(name);
        }

        Expr::EnumDecl(EnumDecl { name, variants }) => {
            log(&format!("Enum tərifi yoxlanılır: '{}'", name));

            if ctx.enum_defs.contains_key(name.as_ref()) {
                return Err(ValidatorError::DuplicateEnum(name.to_string()));
            }

            ctx.enum_defs
                .insert(Cow::Owned(name.to_string()), variants.clone());
        }
        Expr::VariableRef { name, symbol } => {
            log(&format!("Dəmir Əmi dəyişənə baxır: `{}`", name));

            if let Some(sym) = ctx.lookup_variable_mut(name) {
                sym.is_used = true;

                *symbol = Some(sym.clone());

                return Ok(());
            }

            if name == "self" && ctx.current_struct.is_some() {
                *symbol = Some(Symbol {
                    typ: Type::Istifadeci(Cow::Borrowed(ctx.current_struct.unwrap())),
                    is_mutable: false,
                    is_used: true,
                    is_pointer: false,
                });
                return Ok(());
            }

            let is_enum_variant = ctx
                .enum_defs
                .values()
                .any(|variants| variants.contains(name));
            if !is_enum_variant {
                return Err(ValidatorError::UndefinedVariable(name.to_string()));
            }
            return Ok(());
        }
        Expr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            log("Şərt yoxlanılır");

            validate_expr(condition, ctx, log)?;

            let cond_type =
                get_type(condition, ctx, None).ok_or(ValidatorError::IfConditionTypeUnknown)?;
            if cond_type != Type::Bool {
                return Err(ValidatorError::IfConditionTypeMismatch(cond_type));
            }

            for expr in then_branch {
                validate_expr(expr, ctx, log)?;
            }

            for expr in else_branch {
                validate_expr(expr, ctx, log)?;
            }
        }
        Expr::ElseIf {
            condition,
            then_branch,
        } => {
            log("Şərt yoxlanılır");

            validate_expr(condition, ctx, log)?;

            let cond_type =
                get_type(condition, ctx, None).ok_or(ValidatorError::IfConditionTypeUnknown)?;
            if cond_type != Type::Bool {
                return Err(ValidatorError::IfConditionTypeMismatch(cond_type));
            }

            for expr in then_branch {
                validate_expr(expr, ctx, log)?;
            }
        }
        Expr::Else { then_branch } => {
            log("Şərt yoxlanılır");

            for expr in then_branch {
                validate_expr(expr, ctx, log)?;
            }
        }

        Expr::Loop {
            var_name,
            iterable,
            body,
        } => {
            log("Dövr yoxlanılır");
            validate_expr(iterable, ctx, log)?;
            let iterable_type =
                get_type(iterable, ctx, None).ok_or(ValidatorError::LoopIterableTypeNotFound)?;
            if let Type::Siyahi(inner) = iterable_type {
                let symbol = Symbol {
                    typ: *inner,
                    is_mutable: false,
                    is_used: false,
                    is_pointer: false,
                };
                ctx.declare_variable(var_name.to_string(), symbol);
            } else {
                return Err(ValidatorError::LoopRequiresList);
            }
            for expr in body {
                validate_expr(expr, ctx, log)?;
            }
        }

        Expr::TemplateString(chunks) => {
            log("Template string yoxlanılır");
            for chunk in chunks.iter_mut() {
                match chunk {
                    TemplateChunk::Literal(_) => {}
                    TemplateChunk::Expr(expr) => {
                        validate_expr(expr, ctx, log)?;
                    }
                }
            }
        }
        Expr::Call {
            target,
            args,
            returned_type,
            name,
        } => {
            match target {
                Some(variable) => {
                    validate_expr(variable, ctx, log)?;
                    let variable_type = get_type(variable, ctx, None);

                    match variable_type {
                        Some(Type::Metn) => {
                            let union = ctx
                                .union_defs
                                .get("Yazı")
                                .ok_or(ValidatorError::UnionNotFound("Yazı".to_string()))?;
                            let maybe_method = union
                                .1
                                .iter()
                                .find(|m| m.name.to_string() == name.to_string());
                            let method = maybe_method
                                .ok_or_else(|| ValidatorError::FunctionNotFound(name))?;
                            /* TODO: Burada parametr ve args qiymetini yoxla */
                            if method.parameters.len() != args.len() {
                                return Err(ValidatorError::FunctionArgCountMismatch {
                                    name: name.to_string(),
                                    expected: method.parameters.len(),
                                    found: args.len(),
                                });
                            }

                            *returned_type = method.return_type.clone();
                        }
                        Some(Type::Natural) | Some(Type::Integer) | Some(Type::Float) => {
                            let object = ctx
                                .union_defs
                                .get("Ədəd")
                                .ok_or(ValidatorError::UnionNotFound("Ədəd".to_string()))?;
                            let maybe_method = object
                                .1
                                .iter()
                                .find(|m| m.name.to_string() == name.to_string());
                            let method = maybe_method
                                .ok_or_else(|| ValidatorError::FunctionNotFound(name))?;
                            /* TODO: Burada parametr ve args qiymetini yoxla */
                            if method.parameters.len() != args.len() {
                                return Err(ValidatorError::FunctionArgCountMismatch {
                                    name: name.to_string(),
                                    expected: method.parameters.len(),
                                    found: args.len(),
                                });
                            }

                            *returned_type = method.return_type.clone();
                        }
                        Some(Type::Istifadeci(s)) => {
                            let union = ctx
                                .union_defs
                                .get(&s.to_string())
                                .or_else(|| ctx.struct_defs.get(&s.to_string()))
                                .ok_or(ValidatorError::UnionNotFound(s.to_string()))?;
                            let maybe_method = union
                                .1
                                .iter()
                                .find(|m| m.name.to_string() == name.to_string());
                            let method = maybe_method.ok_or_else(|| {
                                ValidatorError::FunctionNotFound(name) // Əgər ayrıca MethodNotFound error varsa onu istifadə et
                            })?;
                            if method.parameters.len() != args.len() + 1 {
                                return Err(ValidatorError::FunctionArgCountMismatch {
                                    name: name.to_string(),
                                    expected: method.parameters.len(),
                                    found: args.len(),
                                });
                            }

                            *returned_type = method.return_type.clone();
                        }
                        _ => {
                            return Err(ValidatorError::UnionNotFound(
                                "Enum tapılmadı".to_string(),
                            ));
                        }
                    }
                }
                _ => {
                    let func = ctx
                        .functions
                        .get(&Cow::Owned(name.to_string()))
                        .ok_or(ValidatorError::FunctionNotFound(name))?;
                    log(&format!("Funksiya çağırışı yoxlanılır: {}", name));
                    if func.parameters.len() != args.len() {
                        return Err(ValidatorError::FunctionArgCountMismatch {
                            name: name.to_string(),
                            expected: func.parameters.len(),
                            found: args.len(),
                        });
                    }

                    *returned_type = func.return_type.clone();
                }
            }
            for arg in args.iter_mut() {
                validate_expr(arg, ctx, log)?;
            }
        }
        Expr::Index {
            target,
            index,
            target_type,
        } => {
            log("Dəmir Əmi indeksləmə əməliyyatını yoxlayır...");

            validate_expr(target, ctx, log)?;
            validate_expr(index, ctx, log)?;
            /*             validate_expr(index, ctx, log)?;
             */
            let index_type = get_type(index, ctx, None);

            if index_type.is_none() {
                return Err(ValidatorError::IndexTargetTypeNotFound);
            }
            let index_type = index_type.unwrap();

            if index_type == Type::Integer {
                *target_type = Type::Integer;
            }
            log("Dəmir Əmi indeksləmə2  əməliyyatını yoxlayır...");

            match index_type {
                Type::Integer => {
                    *target_type = Type::Integer;
                }
                Type::Metn => {
                    log("Dəmir Əmi indeksləmə2  əməliyyatını yoxlayır...");
                    let index_name = match &**index {
                        Expr::String(s, _) => s,
                        _ => return Err(ValidatorError::IndexTargetTypeNotFound),
                    };
                    let struct_type = get_type(target, ctx, None);

                    println!("Sruktur tipi {target:?}");
                    let struct_name = match struct_type {
                        Some(Type::Istifadeci(name)) => name,
                        _ => return Err(ValidatorError::IndexTargetTypeNotFound),
                    };

                    let struct_def = ctx
                        .struct_defs
                        .get(&struct_name.to_string())
                        .or_else(|| ctx.union_defs.get(&struct_name.to_string()))
                        .ok_or(ValidatorError::IndexTargetTypeNotFound)?
                        .0
                        .clone();

                    match &**index {
                        Expr::String(index_name, _) => {
                            log(&format!("Dəmir Əmi sindeksləmə əməliyyatını yoxlayır..."));
                            for (fname, ftype) in struct_def {
                                if fname == *index_name {
                                    *target_type = ftype.clone();
                                    break;
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Expr::BinaryOp { left, op, right } => {
            validate_expr(left, ctx, log)?;
            validate_expr(right, ctx, log)?;
        }
        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } => {
            log(&format!("Funksiya tərifi yoxlanılır: {}", name));
            if ctx.current_function.is_some() {
                return Err(ValidatorError::NestedFunctionDefinition);
            }
            if let Some(Type::Istifadeci(name)) = return_type {
                match ctx.validate_user_type(name) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            ctx.current_function = Some(name.to_string());

            ctx.push_scope();

            for param in params.iter_mut() {
                log(&format!("Parametri yoxlanılır: {}", param.name));
                param.is_pointer = param.is_mutable;
                let symbol = Symbol {
                    typ: param.typ.clone(),
                    is_mutable: param.is_mutable,
                    is_used: false,
                    is_pointer: param.is_pointer,
                };
                ctx.declare_variable(param.name.clone(), symbol);
            }

            let mut owned_body = std::mem::take(body);

            ctx.functions.insert(
                Cow::Borrowed(*name),
                FunctionInfo {
                    name: Cow::Borrowed(*name),
                    parameters: params.clone(),
                    return_type: return_type.clone(),
                },
            );

            for expr in owned_body.iter_mut() {
                validate_expr(expr, ctx, log)?;
            }
            ctx.functions.insert(
                Cow::Borrowed(*name),
                FunctionInfo {
                    name: Cow::Borrowed(*name),
                    parameters: params.clone(),
                    return_type: return_type.clone(),
                },
            );

            ctx.pop_scope();
            ctx.current_function = None;
            ctx.current_return = None;
            *body = owned_body;
        }
        Expr::Return(value) => {
            validate_expr(value, ctx, log)?;
        }
        _ => {}
    }
    Ok(())
}
