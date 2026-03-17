use core::panic;

use parser::{
    ast::{Expr, Symbol},
    shared_ast::Type,
};

use crate::{
    Validator, errors::ValidatorError, function_call, helper::get_type, validate::validate_expr,
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
            symbol,
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
            *symbol = Some(Symbol {
                is_mutable: false,
                typ: Type::Function,
                is_pointer: false,
                is_used: true,
                is_changed: false,
            });
            println!("{:#?}",func.return_type);
            *return_type = func.return_type.clone();

        }
        Expr::Call { target, name, args, returned_type } => {
            validate_function_call(ctx, target, args, returned_type, name)?;
            *returned_type = Some(Type::Integer);/*BUG:   */
        }
        other => {
            panic!("{other:?}");  //BUG: Açıq
        }
    }
    match target {
        Some(variable) => {
            validate_expr(variable, ctx)?;
            let variable_type = get_type(variable, ctx, None);
        }
        None => {}
    }
    for arg in args.iter_mut() {
        validate_expr(arg, ctx)?;
    }
    *return_type = Some(Type::Integer);
    Ok(())
}
