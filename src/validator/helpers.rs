use crate::parser::ast::{Expr, Type};

/* pub fn validate_decl<'a>(
    name: &'a str,
    typ: &'a mut Option<Type<'a>>,
    value: &'a Expr<'a>, // <- no longer &mut
    is_mutable: &'a bool,
    ctx: &mut ValidatorContext<'a>,
    message: &mut dyn FnMut(&str),
) -> Result<Type<'a>, ValidatorError<'a>> {
    message(&format!(
        "{} yaradılır: '{}'",
        if *is_mutable { "Dəyişən" } else { "Sabit" },
        name
    ));

    let inferred = match get_type(value, ctx) {
        Some(t) => t,
        None => {
            if typ.is_none() {
                return Err(ValidatorError::TypeInferenceFailed(name.into()));
            }

            if let Some(Type::Istifadeci(enum_name)) = typ {
                if let Expr::VariableRef {
                    name: variant_name, ..
                } = value
                {
                    if let Some(variants) = ctx.enum_defs.get(enum_name) {
                        if variants.variants.contains(&variant_name.as_ref()) {
                            Type::Istifadeci(enum_name.clone())
                        } else {
                            return Err(ValidatorError::UnknownEnumVariant {
                                enum_name,
                                variant: variant_name,
                            });
                        }
                    } else {
                        return Err(ValidatorError::EnumNotFound(enum_name));
                    }
                } else {
                    return Err(ValidatorError::NotAnEnumVariant);
                }
            } else {
                return Err(ValidatorError::CouldNotInferType(name.into()));
            }
        }
    };

    let declared = typ.clone().unwrap_or_else(|| inferred.clone());

    if inferred != declared {
        return Err(ValidatorError::DeclTypeMismatch {
            name: name.into(),
            expected: declared.clone(),
            found: inferred,
        });
    }

    ctx.declare_variable(
        name.to_string(),
        Symbol {
            typ: declared.clone(),
            is_mutable: *is_mutable,
            is_used: false,
            is_pointer: false,
        },
    );

    // We no longer call validate_expr(value, …) here
    Ok(declared)
} */
/* pub fn sanitize_name(name: &str) -> String {
    let mut result = String::from("az_");

    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            result.push(ch);
        } else {
            result.push_str(&format!("_u{:x}", ch as u32));
        }
    }

    result
}
 */
pub fn get_type<'a>(value: &Expr<'a>) -> Option<Type<'a>> {
    match value {
        Expr::Number(_) => Some(Type::Integer),
        Expr::Bool(_) => Some(Type::Bool),
        Expr::String(_) => Some(Type::Metn),
        Expr::List(_) => Some(Type::Siyahi(Box::new(Type::Any))),
        Expr::Index { target, index } => {
            let target_type = get_type(target);
            let _index_type = get_type(index);
            if let Some(target_type) = target_type {
                if target_type == Type::Siyahi(Box::new(Type::Any)) {
                    return Some(Type::Any);
                }
            }
            None
        }
        Expr::BuiltInCall { return_type, .. } => Some(return_type.clone()),
        Expr::Call { returned_type, .. } => returned_type.clone(),
        _ => None,
    }
}

/*
pub fn get_type<'a>(expr: &Expr<'a>, ctx: &ValidatorContext<'a>) -> Option<Type<'a>> {
    use Expr::*;

    match expr {
        Float(_) => Some(Type::Float),
        Number(_) => Some(Type::Integer),
        Bool(_) => Some(Type::Bool),
        String(s) => {
            if s.len() == 1 {
                Some(Type::Char)
            } else {
                Some(Type::Metn)
            }
        }
        List(items) => {
            if items.is_empty() {
                Some(Type::Siyahi(Box::new(Type::Any)))
            } else {
                let first_type = get_type(&items[0], ctx)?;
                if items
                    .iter()
                    .all(|e| get_type(e, ctx) == Some(first_type.clone()))
                {
                    Some(Type::Siyahi(Box::new(first_type)))
                } else {
                    Some(Type::Siyahi(Box::new(Type::Any)))
                }
            }
        }
        Index { target, .. } => {
            let t = get_type(target, ctx)?;
            match t {
                Type::Siyahi(inner) => Some(*inner),
                Type::Metn => Some(Type::Char),
                _ => None,
            }
        }
        VariableRef { name, .. } => {
            if name == "self" {
                ctx.current_struct
                    .as_ref()
                    .map(|s| Type::Istifadeci(Cow::Owned(s.clone())))
            } else if let Some(sym) = ctx.lookup_variable(name) {
                Some(sym.typ)
            } else if ctx.enum_defs.contains_key(name) {
                Some(Type::Istifadeci(Cow::Owned(name.to_string())))
            } else {
                None
            }
        }
        /*       FieldAccess { target, field, .. } => {
                    if let Some(Type::Istifadeci(struct_name)) = get_type(target, ctx) {
                        if let Some((fields, _methods)) = ctx.struct_defs.get(&struct_name) {
                            for (fname, ftype) in fields {
                                if fname == field {
                                    return Some(ftype.clone());
                                }
                            }
                        }
                    }
                    None
                } */
        /*         StructInit { name, .. } => Some(Type::Istifadeci(name.clone())),
         */        /*         Call { name, .. } => ctx.functions.get(name).and_then(|f| f.return_type.clone()),
                      */
                /*      MethodCall {
                         target,
                         method,
                         args,
                     } => match get_type(target, ctx)? {
                         Type::Siyahi(_) => match method.as_str() {
                             "uzunluq" | "boşdur" => Some(Type::Integer),
                             "sırala" | "əks_sırala" => Some(Type::Siyahi(Box::new(Type::Integer))),
                             _ => None,
                         },
                         Type::Metn => match method.as_str() {
                             "uzunluq" | "boşdur" => Some(Type::Integer),
                             "böyüt" | "kiçilt" | "kənar_təmizlə" | "birləşdir" | "kəs" | "əvəzlə" => {
                                 Some(Type::Metn)
                             }
                             "böl" => Some(Type::Siyahi(Box::new(Type::Metn))),
                             _ => None,
                         },
                         Type::Istifadeci(struct_name) => {
                             if let Some((_fields, methods)) = ctx.struct_defs.get(&struct_name) {
                                 methods
                                     .iter()
                                     .find(|(m, params, _, _)| m == method && args.len() == params.len() - 1)
                                     .and_then(|(_, _, _, ret_type)| ret_type.clone())
                             } else {
                                 None
                             }
                         }
                         _ => None,
                     }, */
        BuiltInCall { function, args, .. } => Some(match function {
            BuiltInFunction::Print => Type::Metn,
            BuiltInFunction::Len
            | BuiltInFunction::Number
            | BuiltInFunction::Sum
            | BuiltInFunction::Sqrt
            | BuiltInFunction::Round
            | BuiltInFunction::Floor
            | BuiltInFunction::Ceil
            | BuiltInFunction::Timer => Type::Integer,
            BuiltInFunction::LastWord | BuiltInFunction::Input => Type::Metn,
            BuiltInFunction::Max | BuiltInFunction::Min | BuiltInFunction::Mod => Type::Integer,
            BuiltInFunction::Range => {
                if args.len() == 2 {
                    let (l, r) = (get_type(&args[0], ctx)?, get_type(&args[1], ctx)?);
                    if l == Type::Integer && r == Type::Integer {
                        Type::Siyahi(Box::new(Type::Integer))
                    } else {
                        Type::Any
                    }
                } else {
                    Type::Any
                }
            }
        }),
        BinaryOp { left, op, right } => {
            let (lt, rt) = (get_type(left, ctx)?, get_type(right, ctx)?);
            if lt != rt {
                return None;
            } else {
                return Some(lt);
            }
            /* if matches!(
                (*op).as_str(),
                "==" | "!=" | "<" | "<=" | ">" | ">=" | "və" | "vəya"
            ) {
                Some(Type::Bool)
            } else {
                Some(lt)
            } */
        }
        If {
            then_branch,
            else_branch,
            ..
        } => {
            let then_type = then_branch.last().and_then(|e| get_type(e, ctx))?;
            if else_branch.is_empty() {
                Some(Type::Void)
            } else if else_branch
                .iter()
                .all(|e| get_type(e, ctx) == Some(then_type.clone()))
            {
                Some(then_type)
            } else {
                Some(Type::Any)
            }
        }
        /*     Match { target, arms } => {
            let _target_type = get_type(target, ctx)?;
            let arm_types: Vec<_> = arms
                .iter()
                .filter_map(|(_, exprs)| exprs.last().and_then(|e| get_type(e, ctx)))
                .collect();

            if arm_types.is_empty() {
                None
            } else if arm_types.windows(2).all(|w| w[0] == w[1]) {
                Some(arm_types[0].clone())
            } else {
                Some(Type::Any)
            }
        } */
        Return(inner) => get_type(inner, ctx),
        _ => None,
    }
}
 */
