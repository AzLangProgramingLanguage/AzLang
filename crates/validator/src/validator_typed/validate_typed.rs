use std::{borrow::Cow, rc::Rc};

use logging::validator_log;
use parser::{
    shared_ast::{BuiltInFunction, Type},
    typed_ast::{Symbol, TypedExpr, TypedTemplateChunk},
};

use crate::{
    errors::ValidatorError,
    helper::get_type_extented,
    validator_typed::{FunctionInfo, MethodInfo, ValidatorTypedContext},
};
pub fn validate_expr_typed<'a>(
    expr: &mut TypedExpr<'a>,
    ctx: &mut ValidatorTypedContext<'a>,
) -> Result<(), ValidatorError> {
    match expr {
        TypedExpr::Decl {
            name,
            typ,
            is_mutable,
            value,
            transpiled_name,
            is_primitive,
        } => {
            validator_log(&format!("✅ Declarasiya yaradılır: {name}"));
            validator_log(&format!(
                "{} yaradılır: '{}'",
                if *is_mutable { "Dəyişən" } else { "Sabit" },
                name
            ));
            if ctx.lookup_variable(name).is_some() {
                return Err(ValidatorError::AlreadyDecl(name.to_string()));
            }

            validate_expr_typed(value, ctx)?;
            let inferred = get_type_extented(value, ctx, typ.as_deref());
            if let Some(s) = inferred {
                if let Some(typ_ref) = typ.as_deref() {
                    if *typ_ref != s {
                        return Err(ValidatorError::DeclTypeMismatch {
                            name: name.to_string(),
                            expected: s.to_string(),
                            found: typ_ref.to_string(),
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
                        transpiled_name: transpiled_name.clone() /* TODO: Buraya baxarsan yersiz clone var. */,
                        //is_allocator: false,
                    },
                );
            } else {
                return Err(ValidatorError::DeclTypeUnknown(name.to_string()));
            }
        }

        TypedExpr::Assignment {
            name,
            value,
            symbol,
        } => {
            validate_expr_typed(value, ctx)?;
            let inferred = get_type_extented(value, ctx, None);
            if let Some(mut var) = ctx.lookup_variable(name) {
                var.is_used = true;
                if !var.is_mutable {
                    return Err(ValidatorError::AssignmentToImmutableVariable(
                        name.to_string(),
                    ));
                }
                if let Some(s) = inferred {
                    if var.typ != s {
                        return Err(ValidatorError::AssignmentTypeMismatch {
                            name: name.to_string(),
                            expected: s.to_string(),
                            found: var.typ.to_string(),
                        });
                    }
                }
            } else {
                return Err(ValidatorError::UndefinedVariable(name.to_string()));
            }
        }
        TypedExpr::UnionType {
            name,
            fields,
            transpiled_name,
            methods,
        } => {
            validator_log(&format!("✅ Union tərifi yoxlanılır: '{name}'"));
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
                .collect::<Result<_, ValidatorError>>()?;

            let newfields: Vec<(&str, Type)> = fields
                .iter()
                .map(|(name, typ)| (*name, typ.clone()))
                .collect();
            ctx.union_defs
                .insert(name.to_string(), (newfields, method_infos));
            for method in methods.iter_mut() {
                ctx.current_struct = Some(name);
                for expr in &mut method.body {
                    validate_expr_typed(expr, ctx)?;
                }

                if let Some(Type::User(name)) = &mut method.return_type {
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
        TypedExpr::Match { target, arms } => {
            validator_log(&format!("✅ Match ifadəsi yoxlanılır"));
            validate_expr_typed(target, ctx)?;
            for arm in arms {
                for expr in arm.1.iter_mut() {
                    validate_expr_typed(expr, ctx)?;
                }
            }
        }
        TypedExpr::String(_, _)
        | TypedExpr::Float(_)
        | TypedExpr::Bool(_)
        | TypedExpr::Number(_)
        | TypedExpr::UnaryOp { .. } => {}
        TypedExpr::BuiltInCall {
            function,
            args,
            return_type: _,
        } => {
            validator_log(&format!("✅ Built-in funksiya yoxlanılır: {function:?}"));

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
                    validate_expr_typed(&mut args[0], ctx)?;
                    validator_log(&format!("✅ Print funksiyası yoxlanılır"));
                    if let Some(t) = get_type_extented(&args[0], ctx, None) {
                        if t == Type::Void {
                            return Err(ValidatorError::TypeMismatch {
                                expected: "Yazı".to_string(),
                                found: format!("{t:?}"),
                            });
                        }
                    }
                }
                BuiltInFunction::ConvertString => {
                    validator_log(&format!("✅ ConvertString funksiyası yoxlanılır"));
                }

                BuiltInFunction::StrUpper
                | BuiltInFunction::StrLower
                | BuiltInFunction::StrReverse => {
                    validator_log(&format!("✅ StrUpper funksiyası yoxlanılır"));
                    if let Some(t) = get_type_extented(&args[0], ctx, None) {
                        if t != Type::String {
                            return Err(ValidatorError::TypeMismatch {
                                expected: Type::String.to_string(),
                                found: format!("{t:?}"),
                            });
                        }
                    }
                }

                BuiltInFunction::Len => {
                    if let Some(t) = get_type_extented(&args[0], ctx, None) {
                        match t {
                            Type::Array(_) => {}
                            _ => {
                                return Err(ValidatorError::TypeMismatch {
                                    expected: "Array".to_string(), /* TODO: HardCode */
                                    found: format!("{t:?}"),
                                });
                            }
                        }
                    }
                    if args.len() != 1 {
                        return Err(ValidatorError::InvalidArgumentCount {
                            name: function.to_string(),
                            expected: 1,
                            found: args.len(),
                        });
                    }
                }
                _ => {}
            }
            for arg in args {
                validate_expr_typed(arg, ctx)?;
            }
        }
        TypedExpr::StructInit {
            name,
            args,
            transpiled_name,
        } => {
            validator_log(&format!("✅ Struct yoxlanılır: '{}'", name));

            if let Some((s, ..)) = ctx.struct_defs.get(name.as_ref()) {
            } else if let Some((s, ..)) = ctx.union_defs.get(name.as_ref()) {
            } else {
                return Err(ValidatorError::UnknownStruct(name.to_string()));
            }

            for arg in args.iter_mut() {
                validate_expr_typed(&mut arg.1, ctx)?;
            }
        }
        TypedExpr::StructDef {
            name,
            fields,
            transpiled_name,
            methods,
        } => {
            validator_log(&format!("✅ Struct tərifi yoxlanılır: '{}'", name));
            if ctx.struct_defs.contains_key(*name) {
                return Err(ValidatorError::DuplicateStruct(name.to_string()));
            }

            let method_infos = methods
                .iter()
                .map(|method| {
                    let mut cloned_ret_type = method.return_type.clone();
                    if let Some(Type::User(name)) = &mut cloned_ret_type {
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
                .collect::<Result<Vec<_>, ValidatorError>>()?;

            let newfields: Vec<(&str, Type)> = fields
                .iter()
                .map(|(name, typ, _)| (*name, typ.clone()))
                .collect();
            ctx.struct_defs
                .insert(name.to_string(), (newfields, method_infos));
            for method in methods.iter_mut() {
                ctx.current_struct = Some(name);
                for expr in &mut method.body {
                    validate_expr_typed(expr, ctx)?;
                }
                validator_log(&format!(
                    "✅ Struct metodları yoxlanılır: '{}'",
                    method.name
                ));

                ctx.is_allocator_used = false;
                ctx.current_struct = None;
            }
        }

        TypedExpr::EnumDecl { name, variants } => {
            validator_log(&format!("Enum tərifi yoxlanılır: '{}'", name));

            if ctx.enum_defs.contains_key(name.as_ref()) {
                return Err(ValidatorError::DuplicateEnum(name.to_string()));
            }

            ctx.enum_defs
                .insert(Cow::Owned(name.to_string()), variants.clone());
        }
        TypedExpr::VariableRef {
            name,
            symbol,
            transpiled_name,
        } => {
            validator_log(&format!("Dəmir Əmi dəyişənə baxır: `{}`", name));

            if let Some(sym) = ctx.lookup_variable_mut(name) {
                sym.is_used = true;

                *symbol = Some(sym.clone());

                return Ok(());
            }

            if name == "self" && ctx.current_struct.is_some() {
                *symbol = Some(Symbol {
                    typ: Type::User(Cow::Borrowed(ctx.current_struct.unwrap())),
                    is_mutable: false,
                    is_used: true,
                    is_pointer: false,
                    transpiled_name: transpiled_name.clone(), /* TODO: buraya baxarsan burada clone var/ */
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
        TypedExpr::If {
            condition,
            then_branch,
            else_branch,
        } => {
            validator_log("Şərt yoxlanılır");

            validate_expr_typed(condition, ctx)?;

            let cond_type = get_type_extented(condition, ctx, None)
                .ok_or(ValidatorError::IfConditionTypeUnknown)?;
            if cond_type != Type::Bool {
                return Err(ValidatorError::IfConditionTypeMismatch(
                    cond_type.to_string(),
                ));
            }

            for expr in then_branch {
                validate_expr_typed(expr, ctx)?;
            }

            for expr in else_branch {
                validate_expr_typed(expr, ctx)?;
            }
        }
        TypedExpr::ElseIf {
            condition,
            then_branch,
        } => {
            validator_log("Şərt yoxlanılır");

            validate_expr_typed(condition, ctx)?;

            let cond_type = get_type_extented(condition, ctx, None)
                .ok_or(ValidatorError::IfConditionTypeUnknown)?;
            if cond_type != Type::Bool {
                return Err(ValidatorError::IfConditionTypeMismatch(
                    cond_type.to_string(),
                ));
            }

            for expr in then_branch {
                validate_expr_typed(expr, ctx)?;
            }
        }
        TypedExpr::Else { then_branch } => {
            validator_log("Şərt yoxlanılır");

            for expr in then_branch {
                validate_expr_typed(expr, ctx)?;
            }
        }

        TypedExpr::Loop {
            var_name,
            iterable,
            body,
        } => {
            validator_log("Dövr yoxlanılır");
            validate_expr_typed(iterable, ctx)?;
            let iterable_type = get_type_extented(iterable, ctx, None)
                .ok_or(ValidatorError::LoopIterableTypeNotFound)?;
            if let Type::Array(inner) = iterable_type {
                let symbol = Symbol {
                    typ: *inner,
                    is_mutable: false,
                    is_used: false,
                    is_pointer: false,
                    transpiled_name: None, /* TODO: buraya baxarsan burada problem var./ */
                };
                ctx.declare_variable(var_name.to_string(), symbol);
            } else {
                return Err(ValidatorError::LoopRequiresList);
            }
            for expr in body {
                validate_expr_typed(expr, ctx)?;
            }
        }

        TypedExpr::TemplateString(chunks) => {
            validator_log("Template string yoxlanılır");
            for chunk in chunks.iter_mut() {
                match chunk {
                    TypedTemplateChunk::Literal(_) => {}
                    TypedTemplateChunk::TypedExpr(expr) => {
                        validate_expr_typed(expr, ctx)?;
                    }
                }
            }
        }
        TypedExpr::Call {
            target,
            args,
            returned_type,
            name,
            transpiled_name,
            is_allocator,
        } => {
            match target {
                Some(variable) => {
                    validate_expr_typed(variable, ctx)?;
                    let variable_type = get_type_extented(variable, ctx, None);

                    match variable_type {
                        Some(Type::String) => {
                            let union = ctx
                                .union_defs
                                .get("Yazı")
                                .ok_or(ValidatorError::UnionNotFound("Yazı".to_string()))?;
                            let maybe_method = union
                                .1
                                .iter()
                                .find(|m| m.name.to_string() == name.to_string());
                            let method = maybe_method.ok_or_else(|| {
                                ValidatorError::FunctionNotFound(name.to_string())
                            })?;
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
                            let method = maybe_method.ok_or_else(|| {
                                ValidatorError::FunctionNotFound(name.to_string())
                            })?;
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
                        Some(Type::User(s)) => {
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
                                ValidatorError::FunctionNotFound(name.to_string())
                                // Əgər ayrıca MethodNotFound error varsa onu istifadə et
                            })?;
                            if method.parameters.len() != args.len() {
                                return Err(ValidatorError::FunctionArgCountMismatch {
                                    name: name.to_string(),
                                    expected: method.parameters.len(),
                                    found: args.len(),
                                });
                            }

                            *returned_type = method.return_type.clone();
                        }
                        Some(Type::Array(_)) => {
                            let union = ctx
                                .union_defs
                                .get("Siyahı")
                                .or_else(|| ctx.struct_defs.get("Siyahı"))
                                .ok_or(ValidatorError::UnionNotFound("Siyahı".to_string()))?;
                            let maybe_method = union
                                .1
                                .iter()
                                .find(|m| m.name.to_string() == name.to_string());
                            let method = maybe_method.ok_or_else(|| {
                                ValidatorError::FunctionNotFound(name.to_string())
                            })?;
                            if method.parameters.len() != args.len() {
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
                        .ok_or(ValidatorError::FunctionNotFound(name.to_string()))?;
                    validator_log(&format!("Funksiya çağırışı yoxlanılır: {}", name));

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
                validate_expr_typed(arg, ctx)?;
            }
        }
        TypedExpr::Index {
            target,
            index,
            target_type,
        } => {
            validator_log("indeksləmə əməliyyatını yoxlayır...");

            validate_expr_typed(target, ctx)?;
            validate_expr_typed(index, ctx)?;
            /*             validate_expr_typed(index, ctx, log)?;
             */
            let index_type = get_type_extented(index, ctx, None);

            if index_type.is_none() {
                return Err(ValidatorError::IndexTargetTypeNotFound);
            }
            let index_type = index_type.unwrap();

            if index_type == Type::Integer {
                *target_type = Type::Integer;
            }
            validator_log("indeksləmə2  əməliyyatını yoxlayır...");

            match index_type {
                Type::Integer => {
                    *target_type = Type::Integer;
                }
                Type::String => {
                    validator_log("indeksləmə2  əməliyyatını yoxlayır...");
                    let index_name = match &**index {
                        TypedExpr::String(s, _) => s,
                        _ => return Err(ValidatorError::IndexTargetTypeNotFound),
                    };
                    let struct_type = get_type_extented(target, ctx, None);

                    println!("Sruktur tipi {target:?}");
                    let struct_name = match struct_type {
                        Some(Type::User(name)) => name,
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
                        TypedExpr::String(index_name, _) => {
                            validator_log(&format!("sindeksləmə əməliyyatını yoxlayır..."));
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
        TypedExpr::BinaryOp { left, op, right } => {
            validate_expr_typed(left, ctx)?;
            validate_expr_typed(right, ctx)?;
        }
        TypedExpr::FunctionDef {
            name,
            params,
            body,
            return_type,
            transpiled_name,
            is_allocator,
        } => {
            /*TODO: Burada value+ tip yoxlanılması et */
            validator_log(&format!("Funksiya tərifi yoxlanılır: {}", name));
            if ctx.current_function.is_some() {
                return Err(ValidatorError::NestedFunctionDefinition);
            }
            if let Some(Type::User(name)) = return_type {
                match ctx.validate_user_type(name) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }
            ctx.current_function = Some(name.to_string());

            ctx.push_scope();

            for param in params.iter_mut() {
                validator_log(&format!("Parametri yoxlanılır: {}", param.name));
                param.is_pointer = param.is_mutable;
                let symbol = Symbol {
                    typ: param.typ.clone(),
                    is_mutable: param.is_mutable,
                    is_used: false,
                    is_pointer: param.is_pointer,
                    transpiled_name: None,
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
                validate_expr_typed(expr, ctx)?;
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

        _ => {}
    }
    Ok(())
}
