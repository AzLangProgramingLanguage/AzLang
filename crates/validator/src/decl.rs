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
    typ: Rc<Type>,
    is_mutable: bool,
    value: Expr,
    ctx: &mut Validator,
) -> Result<Ast, ValidatorError> {
    if ctx.lookup_variable(name.as_ref()).is_some() {
        return Err(ValidatorError::AlreadyDecl(name.to_string()));
    }

    let mut inferred = get_type(&value, ctx)?;
    reconcile_type(typ, &mut inferred, name.as_ref())?;

    ctx.declare_variable(
        name.to_string(),
        Symbol {
            typ: inferred.clone(),
            is_used: false,
            is_mutable,
            is_changed: false,
        },
    );
    let val = validate_expr(value, ctx)?;

    Ok(Ast::Decl {
        name: name.to_string(),
        typ: inferred,
        is_mutable,
        value: Box::new(val),
    })
}
