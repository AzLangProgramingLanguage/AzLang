use std::{
    borrow::Cow,
    collections::{HashMap, hash_map::Entry},
    ops::Deref,
    rc::Rc,
};

use logging::validator_log;
use parser::{
    ast::{Expr, Parameter, Symbol, TemplateChunk},
    shared_ast::{BuiltInFunction, Type},
};

use crate::{FunctionInfo, MethodInfo, ValidatorContext, errors::ValidatorError, helper::get_type};
pub fn validate_expr<'a>(
    expr: &mut Expr<'a>,
    ctx: &mut ValidatorContext<'a>,
) -> Result<(), ValidatorError> {
    match expr {
        Expr::Decl {
            name,
            typ,
            is_mutable,
            value,
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

            validate_expr(value, ctx)?;
            let inferred = get_type(value, ctx, Some(typ));
            if *typ == Type::Any.into() {
                *typ = Rc::new(inferred);
            } else if inferred != **typ {
                if inferred == Type::LiteralString && **typ == Type::String {
                    *typ = Rc::new(Type::String);
                } else {
                    return Err(ValidatorError::DeclTypeMismatch {
                        name: name.to_string(),
                        expected: inferred.to_string(),
                        found: typ.to_string(),
                    });
                }
            }
            ctx.declare_variable(
                name.to_string(),
                Symbol {
                    typ: (**typ).clone(),
                    is_mutable: *is_mutable,
                    is_used: false,
                    is_pointer: false,
                    is_changed: false,
                },
            );
        }

        Expr::Assignment {
            name,
            value,
            symbol,
        } => {
            validator_log(&format!("✅ Assignment yoxlanılır: '{name}'"));
            validate_expr(value, ctx)?;
            let inferred = get_type(value, ctx, None);
            if let Some(var) = ctx.lookup_variable(name) {
                var.is_used = true;
                var.is_changed = true;

                if !var.is_mutable {
                    return Err(ValidatorError::AssignmentToImmutableVariable(
                        name.to_string(),
                    ));
                }
                if let s = inferred {
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
        Expr::UnionType {
            name,
            fields,
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
                    validate_expr(expr, ctx)?;
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
        Expr::Match { target, arms } => {
            validator_log(&format!("✅ Match ifadəsi yoxlanılır"));
            validate_expr(target, ctx)?;
            for arm in arms {
                for expr in arm.1.iter_mut() {
                    validate_expr(expr, ctx)?;
                }
            }
        }
        Expr::String(_)
        | Expr::Float(_)
        | Expr::Bool(_)
        | Expr::Number(_)
        | Expr::UnaryOp { .. } => {}
        Expr::BuiltInCall {
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
                    validate_expr(&mut args[0], ctx)?;
                    validator_log(&format!("✅ Print funksiyası yoxlanılır"));
                    if let t = get_type(&args[0], ctx, None) {
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
                    if let t = get_type(&args[0], ctx, None) {
                        if t != Type::String {
                            return Err(ValidatorError::TypeMismatch {
                                expected: Type::String.to_string(),
                                found: format!("{t:?}"),
                            });
                        }
                    }
                }

                BuiltInFunction::Len => {
                    if let t = get_type(&args[0], ctx, None) {
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
                validate_expr(arg, ctx)?;
            }
        }
        Expr::StructInit { name, args } => {
            validator_log(&format!("✅ Struct yoxlanılır: '{}'", name));

            if let Some((s, ..)) = ctx.struct_defs.get(name.as_ref()) {
            } else if let Some((s, ..)) = ctx.union_defs.get(name.as_ref()) {
            } else {
                return Err(ValidatorError::UnknownStruct(name.to_string()));
            }

            for arg in args.iter_mut() {
                validate_expr(&mut arg.1, ctx)?;
            }
        }
        Expr::StructDef {
            name,
            fields,
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
                    validate_expr(expr, ctx)?;
                }
                validator_log(&format!(
                    "✅ Struct metodları yoxlanılır: '{}'",
                    method.name
                ));

                ctx.is_allocator_used = false;
                ctx.current_struct = None;
            }
        }

        Expr::EnumDecl { name, variants } => {
            validator_log(&format!("Enum tərifi yoxlanılır: '{}'", name));

            if ctx.enum_defs.contains_key(name.as_ref()) {
                return Err(ValidatorError::DuplicateEnum(name.to_string()));
            }

            ctx.enum_defs
                .insert(Cow::Owned(name.to_string()), variants.clone());
        }
        Expr::VariableRef { name, symbol } => {
            validator_log(&format!("Dəmir Əmi dəyişənə baxır: `{}`", name));

            if let Some(mut sym) = ctx.lookup_variable(name) {
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
                    is_changed: false,
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
        Expr::Condition { main, elif, other } => {
            validator_log("Şərt yoxlanılır");
            //TODO: TOoo Bad Code

            validate_expr(&mut *main.condition, ctx)?;

            let cond_type = match get_type(&mut *main.condition, ctx, None) {
                Type::Any => return Err(ValidatorError::IfConditionTypeUnknown),
                other => other,
            };
            if cond_type != Type::Bool {
                return Err(ValidatorError::IfConditionTypeMismatch(
                    cond_type.to_string(),
                ));
            }

            for expr in main.body.iter_mut() {
                validate_expr(expr, ctx)?;
            }
            for condition in elif {
                let cond_type = match get_type(&mut *main.condition, ctx, None) {
                    Type::Any => return Err(ValidatorError::IfConditionTypeUnknown),
                    other => other,
                };
                if cond_type != Type::Bool {
                    return Err(ValidatorError::IfConditionTypeMismatch(
                        cond_type.to_string(),
                    ));
                }
                for expr in condition.body.iter_mut() {
                    validate_expr(expr, ctx)?;
                }
            }
            if let Some(condition) = other {
                for expr in condition.body.iter_mut() {
                    validate_expr(expr, ctx)?;
                }
            }
        }

        Expr::Loop {
            var_name,
            iterable,
            body,
        } => {
            validator_log("Dövr yoxlanılır");
            validate_expr(iterable, ctx)?;
            let iterable_type = match get_type(iterable, ctx, None) {
                Type::Any => return Err(ValidatorError::LoopIterableTypeNotFound),
                other => other,
            };
            if let Type::Array(inner) = iterable_type {
                let symbol = Symbol {
                    typ: *inner,
                    is_mutable: false,
                    is_used: false,
                    is_pointer: false,
                    is_changed: false,
                };
                ctx.declare_variable(var_name.to_string(), symbol);
            } else {
                return Err(ValidatorError::LoopRequiresList);
            }
            for expr in body {
                validate_expr(expr, ctx)?;
            }
        }

        Expr::TemplateString(chunks) => {
            validator_log("Template string yoxlanılır");
            for chunk in chunks.iter_mut() {
                match chunk {
                    TemplateChunk::Literal(_) => {}
                    TemplateChunk::Expr(expr) => {
                        validate_expr(expr, ctx)?;
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
                    validate_expr(variable, ctx)?;
                    let variable_type = get_type(variable, ctx, None);

                    match variable_type {
                        Type::String => {
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
                        Type::Natural | Type::Integer | Type::Float => {
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
                        Type::User(s) => {
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
                        Type::Array(_) => {
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
                validate_expr(arg, ctx)?;
            }
        }
        Expr::Index {
            target,
            index,
            target_type,
        } => {
            validator_log("indeksləmə əməliyyatını yoxlayır...");

            validate_expr(target, ctx)?;
            validate_expr(index, ctx)?;

            let index_type = get_type(index, ctx, None);

            if index_type == Type::Any {
                return Err(ValidatorError::IndexTargetTypeNotFound);
            }

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
                        Expr::String(s) => s,
                        _ => return Err(ValidatorError::IndexTargetTypeNotFound),
                    };
                    let struct_type = get_type(target, ctx, None);

                    println!("Sruktur tipi {target:?}");
                    let struct_name = match struct_type {
                        Type::User(name) => name,
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
                        Expr::String(index_name) => {
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
        Expr::BinaryOp {
            left,
            right,
            op,
            return_type,
        } => {
            validate_expr(left, ctx)?;
            validate_expr(right, ctx)?;
            let typ = get_type(
                &Expr::BinaryOp {
                    left: Box::new(*left.clone()),
                    right: Box::new(*right.clone()),
                    op,
                    return_type: Type::Any,
                },
                ctx,
                None,
            );
            *return_type = typ;
        }
        Expr::FunctionDef {
            name,
            params,
            body,
            return_type,
        } => {
            validator_log(&format!("Funksiya tərifi yoxlanılır: {}", name));
            if ctx.current_function.is_some() {
                return Err(ValidatorError::NestedFunctionDefinition);
            }
            ctx.current_function = Some(name.to_string());

            let function = match ctx.functions.entry(Cow::Borrowed(name)) {
                Entry::Occupied(_) => {
                    return Err(ValidatorError::FunctionAlreadyDefined(name.to_string()));
                }
                Entry::Vacant(entry) => entry.insert(FunctionInfo {
                    variables: HashMap::new(),
                    return_type: return_type.clone(),
                    parameters: vec![],
                }),
            };

            for param in params.iter_mut() {
                validator_log(&format!("Parametri yoxlanılır: {}", param.name));
                let symbol = Symbol {
                    typ: param.typ.clone(),
                    is_mutable: param.is_mutable,
                    is_used: false,
                    is_pointer: param.is_mutable,
                    is_changed: false,
                };

                function.variables.insert(param.name.clone(), symbol);
            }
            function.parameters = params.clone();

            for expr in body.iter_mut() {
                match expr {
                    Expr::Return(value) => {
                        validate_expr(value, ctx)?;
                        if let Some(typ) = return_type {
                            if typ.clone() != get_type(value, ctx, None) {
                                return Err(ValidatorError::FunctionReturnTypeErr(typ.to_string()));
                            }
                        }
                    }
                    _ => {
                        validate_expr(expr, ctx)?;
                    }
                }
            }
            ctx.current_function = None;
            ctx.current_return = None;
        }

        _ => {}
    }
    Ok(())
}
