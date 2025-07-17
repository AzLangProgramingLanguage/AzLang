use color_eyre::eyre::Result;

use crate::{
    parser::ast::{BuiltInFunction, Expr, Symbol, Type},
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
            let inferred = get_type(value);
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
                    if let Some(t) = get_type(&args[0]) {
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
        }

        _ => return Err(ValidatorError::UnknownExpression),
    }
    Ok(())
}
