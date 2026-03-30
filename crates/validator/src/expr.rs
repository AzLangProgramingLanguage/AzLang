use parser::ast::Expr;

use crate::{Validator, errors::ValidatorError};

pub fn validate_expr(expr: &mut Expr, ctx: &mut Validator) -> Result<(), ValidatorError> {
    match expr {
        Expr::String(_) | Expr::Float(_) | Expr::Bool(_) | Expr::Number(_) => Ok(()),
        Expr::Call {
            target,
            name,
            args,
            returned_type,
        } => Ok(()),
        Expr::VariableRef { name, symbol } => {
            if let Some(var) = ctx.lookup_variable(name) {
                var.is_used = true;
                *symbol = Some(var.clone()); //TODO: Clone etmək əvəzinə reference istifadə etməliyik
                Ok(())
            } else {
                return Err(ValidatorError::UndefinedVariable(name.to_string()));
            }
        }
        Expr::Index {
            target,
            index,
            target_type,
        } => Ok(()),
        _ => todo!("Bura baxmaq lazımdır"),
    }
}
