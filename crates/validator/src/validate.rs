use std::rc::Rc;

use parser::{
    ast::{Expr::BuiltInCall, Statement, Symbol},
    shared_ast::{BuiltInFunction, StringEnum, Type},
};
type ValidatorExpr = crate::ast::Expr;
use crate::{
    Validator,
    ast::{
        Ast::{self},
        Decl, Program,
    },
    errors::ValidatorError,
    expr::validate_expr,
    helper::{get_type, reconcile_type, type_checking},
};
pub fn validate_statement(
    stmt: Statement,
    program: &mut Program,
    ctx: &mut Validator,
) -> Result<(), ValidatorError> {
    match stmt {
        Statement::Decl {
            name,
            typ,
            is_mutable,
            value,
        } => {
            if ctx.lookup_variable(&name).is_some() {
                return Err(ValidatorError::AlreadyDecl(name.to_string()));
            }

            let mut inferred = get_type(&value, ctx);
            inferred = reconcile_type(typ, inferred, &name)?;

            ctx.declare_variable(
                name.to_string(),
                Symbol {
                    typ: inferred.clone(),
                    is_used: false,
                    is_pointer: is_mutable,
                    is_changed: false,
                },
            );
            let val = validate_expr(*value, ctx)?;

            program.expressions.push(Ast::Decl(Decl {
                name,
                typ: inferred,
                value: Box::new(val),
            }));
        }
        Statement::Assignment { name, value } => {
            let inferred = get_type(&value, ctx);
            let symbol = ctx.lookup_variable_mut_with_err(&name)?;
            symbol.is_changed = true;
            if !symbol.is_pointer {
                return Err(ValidatorError::AssignmentToImmutableVariable(name));
            }

            type_checking(symbol.typ.clone(), inferred)?;

            let val = validate_expr(*value, ctx)?;

            program.expressions.push(Ast::Assign(name, Box::new(val)));
        }
        Statement::Expr(expr) => {
            let expr = validate_expr(expr, ctx)?;
            program.expressions.push(Ast::Expr(expr));
        }
        _ => {}
    }
    Ok(())
}
