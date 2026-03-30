use core::panic;

use parser::{
    ast::{Expr, Symbol},
    shared_ast::Type,
};

use crate::{
    Validator, errors::ValidatorError, expr::validate_expr, function_call, helper::get_type,
};

pub fn validate_function_call(
    ctx: &mut Validator,
    target: &mut Option<Box<Expr>>,
    args: &mut Vec<Expr>,
    return_type: &mut Option<Type>,
    name: &mut Box<Expr>,
) -> Result<(), ValidatorError> {
    match &mut **name {
        Expr::VariableRef {
            name: func_name,
            symbol:
                Some(Symbol {
                    typ: Type::Function,
                    ..
                }),
        } => {
            let func = ctx
                .functions
                .get(func_name)
                .ok_or(ValidatorError::FunctionNotFound(func_name.to_string()))?;
            if func.parameters.len() != args.len() {
                return Err(ValidatorError::FunctionArgCountMismatch {
                    name: format!("{func_name:?}"),
                    expected: func.parameters.len(),
                    found: args.len(),
                });
            }
            *return_type = func.return_type.clone();
        }
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => {
            validate_function_call(ctx, target, args, returned_type, name)?;
            *returned_type = Some(Type::Function);
        }
        other => {
            return Err(ValidatorError::InvalidFunctionCall(other.to_string()));
        }
    }
    if let Some(variable) = target {
        //BUG: Hələ method çağırışlarını dəstəkləmədiyimiz üçün bu hissə hələ implementasiya edilməyib. Gələcəkdə method çağırışlarını dəstəkləmək üçün istifadə olunacaq.
        panic!(
            "Burası hələ implementasiya etməmişik, amma gələcəkdə method çağırışlarını dəstəkləmək üçün istifadə olunacaq."
        );
    }

    for arg in args.iter_mut() {
        validate_expr(arg, ctx)?;
    }
    *return_type = Some(Type::Integer);
    Ok(())
}
