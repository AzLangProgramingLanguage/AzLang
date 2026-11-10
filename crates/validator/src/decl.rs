use std::rc::Rc;

use parser::{
    ast::{Atom, Expr, Symbol},
    shared_ast::Type,
};

use crate::{
    Validator,
    ast::Ast,
    errors::ValidatorError,
    expr::validate_expr,
    helper::{get_type, reconcile_type},
};

pub fn validate_decl(
    name: Atom,
    typ: Type,
    is_mutable: bool,
    value: Rc<Box<Expr>>,
    ctx: &mut Validator,
) -> Result<(), ValidatorError> {
    if ctx.lookup_variable(&name).is_some() {
        return Err(ValidatorError::AlreadyDecl(name.to_string()));
    }

    let mut inferred = get_type(&value, ctx)?;
    reconcile_type(typ, &mut inferred)?;

    let val = validate_expr(**value.clone(), ctx)?; //TODO 

    ctx.declare_variable(
        name.clone(),
        Symbol {
            typ: inferred.clone(),
            is_used: false,
            is_mutable,
            val: value,
            is_changed: false,
        },
    );
    Ok(())
}

pub fn validate_assignment(
    ctx: &mut Validator,
    name: Atom,
    val: Box<Expr>,
) -> Result<(), ValidatorError> {
    let inferred = get_type(&val, ctx)?;
    let symbol = ctx.lookup_variable_mut_with_err(&name)?;
    symbol.is_changed = true;
    if !symbol.is_mutable {
        return Err(ValidatorError::AssignmentToImmutableVariable(
            name.to_string(),
        ));
    }

    reconcile_type(symbol.typ, inferred)?;

    let val = validate_expr(*value, ctx)?;
    Ok(())
}
