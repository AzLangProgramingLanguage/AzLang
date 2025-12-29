use parser::ast::{Expr, Program};
use validator::ValidatorContext;

pub fn clean_ast<'a>(program: &mut Program<'a>, ctx: &ValidatorContext<'a>) {
    program.expressions.retain(|expr| match expr {
        Expr::Decl { name, .. } => {
            let name_str: &str = name.as_ref();
            ctx.global_variables.get(name_str).unwrap().is_used
        }
        _ => true,
    });
}
