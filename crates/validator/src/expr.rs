use parser::{
    ast::{Expr, TemplateChunk},
    shared_ast::Type,
};

use crate::{Validator, errors::ValidatorError, helper::get_type};

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
        Expr::List(ex) => {
            let mut iter = ex.iter();

            let typ = match iter.next() {
                Some(first) => get_type(first, ctx),
                None => Type::Any,
            };

            for e in iter {
                let current_type = get_type(e, ctx);

                if current_type != typ {
                    return Err(ValidatorError::TypeMismatch {
                        expected: typ,
                        found: current_type,
                    });
                }
            }

            Ok(())
        }
        Expr::BinaryOp {
            left,
            right,
            op,
            return_type,
        } => {
            validate_expr(left, ctx)?;
            validate_expr(right, ctx)?;
            let returned_type = get_type(
                &Expr::BinaryOp {
                    left: left.clone(),
                    right: right.clone(),
                    op: *op,
                    return_type: Type::Any,
                },
                ctx,
            );
            *return_type = returned_type;
            Ok(())
        }
        Expr::Index {
            target,
            index,
            target_type,
        } => Ok(()),
        Expr::TemplateString(chunk) => {
            for ch in chunk {
                if let TemplateChunk::Expr(expr) = ch {
                    validate_expr(expr, ctx)?;
                }
            }
            Ok(())
        }
        other => {
            println!("Bura baxmaq lazımdır {:#?}", other);
            todo!("Bura baxmaq lazımdır")
        }
    }
}
