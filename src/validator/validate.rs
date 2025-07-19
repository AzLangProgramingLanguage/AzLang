use std::borrow::Cow;

use color_eyre::eyre::Result;

use crate::{
    parser::ast::{BuiltInFunction, EnumDecl, Expr, Symbol, TemplateChunk, Type},
    translations::validator_messages::ValidatorError,
    validator::{ValidatorContext, helpers::get_type},
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
            let inferred = get_type(value, ctx);
            if let Some(s) = inferred {
                if let Some(typ_ref) = typ {
                    if *typ_ref != s {
                        return Err(ValidatorError::DeclTypeMismatch {
                            name: name.to_string(),
                            expected: format!("{s:?}"),
                            found: format!("{typ_ref:?}"),
                        });
                    }
                }
                *typ = Some(s.clone());

                ctx.declare_variable(
                    name.to_string(),
                    Symbol {
                        typ: s,
                        is_mutable: *is_mutable,
                        is_used: false,
                        is_pointer: false,
                    },
                );
            }
            validate_expr(value, ctx, log)?;
        }
        Expr::String(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Number(_) => {}
        Expr::BuiltInCall {
            function,
            args,
            return_type,
        } => {
            log(&format!("✅ Built-in funksiya yoxlanılır: {function:?}"));
            match function {
                f if f.expected_arg_count().is_some() => {
                    let expected = f.expected_arg_count().unwrap();
                    if args.len() != expected {
                        return Err(ValidatorError::InvalidArgumentCount {
                            name: f.to_string(),
                            expected,
                            found: args.len(),
                        });
                    }
                }
                BuiltInFunction::Len => {
                    if let Some(t) = get_type(&args[0], ctx) {
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
                _ => todo!(),
            }
            for arg in args {
                validate_expr(arg, ctx, log)?;
            }
        }
        Expr::EnumDecl(EnumDecl { name, variants }) => {
            log(&format!("Enum tərifi yoxlanılır: '{}'", name));

            if ctx.enum_defs.contains_key(&name.to_string()) {
                return Err(ValidatorError::DuplicateEnum(name.to_string()));
            }

            ctx.enum_defs.insert(name.to_string(), variants.clone());
        }
        Expr::VariableRef { name, symbol } => {
            log(&format!("Dəmir Əmi dəyişənə baxır: `{}`", name));

            // Əgər dəyişən scope içində tapılırsa, symbol əlavə olunur
            if let Some(found_symbol) = ctx.lookup_variable(name) {
                *symbol = Some(found_symbol.clone());
                /*    *name = found_symbol.transpile_name.clone(); */
                return Ok(());
            }

            if name == "self" && ctx.current_struct.is_some() {
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
                get_type(condition, ctx).ok_or(ValidatorError::IfConditionTypeUnknown)?;
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
                get_type(condition, ctx).ok_or(ValidatorError::IfConditionTypeUnknown)?;
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
                get_type(iterable, ctx).ok_or(ValidatorError::LoopIterableTypeNotFound)?;
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
            if target.is_some() {
                validate_expr(target.as_mut().unwrap(), ctx, log)?;
                /*  let target_type = get_type(target.as_mut().unwrap(), ctx)
                .ok_or(ValidatorError::FunctionTargetTypeNotFound)?; */
            }
            log(&format!("Funksiya çağırışı yoxlanılır: {}", name));
            let func = ctx
                .functions
                .get(&Cow::Owned(name.to_string()))
                .ok_or(ValidatorError::FunctionNotFound(&name))?;

            if func.parameters.len() != args.len() {
                return Err(ValidatorError::FunctionArgCountMismatch {
                    name: name.to_string(),
                    expected: func.parameters.len(),
                    found: args.len(),
                });
            }
            *returned_type = func.return_type.clone();

            for arg in args.iter_mut() {
                validate_expr(arg, ctx, log)?;
            }

            /*     for (param, arg) in func.parameters.iter().zip(args.iter_mut()) {
                if param.is_pointer {
                    if let Expr::VariableRef {
                        symbol: Some(sym), ..
                    } = arg
                    {
                        sym.is_pointer = true;
                    }
                }
            } */
            /*             *returned_type = func.return_type.clone();
             */ /*      log(&format!("Funksiya çağırış yoxlanılır: {}", target));
            

            validate_expr(target, ctx, log)?;

            for arg in args {
            validate_expr(arg, ctx, log)?;
            } */
        }
        Expr::Index { target, index } => {
            log("Dəmir Əmi indeksləmə əməliyyatını yoxlayır...");

            validate_expr(target, ctx, log)?;
            validate_expr(index, ctx, log)?;
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
            for expr in body {
                validate_expr(expr, ctx, log)?;
            }
            ctx.pop_scope();
            ctx.current_function = None;
            ctx.current_return = None;
        }
        Expr::Return(value) => {
            validate_expr(value, ctx, log)?;
        }
        _ => {}
    }
    Ok(())
}
