use parser::ast::{Expr, Program};
use validator::Validator;

pub fn clean_ast<'a>(program: &mut Program, ctx: &Validator) {
    program.expressions.retain(|expr| match expr {
        Expr::Decl { name, .. } => {
            let name_str: &str = name.as_ref();
            ctx.global_variables.get(name_str).unwrap().is_used
        }
        _ => true,
    });
}
