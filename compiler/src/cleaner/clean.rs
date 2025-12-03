use parser::ast::{Expr, Program};
use validator::ValidatorContext;

pub fn clean_ast<'a>(program: &mut Program<'a>, ctx: &ValidatorContext<'a>) {
    program.expressions.retain(|expr| match expr {
        Expr::Decl { name, .. } => {
            let name_str: &str = name.as_ref();
            ctx.scopes
                .iter()
                .any(|scope| scope.get(name_str).map_or(false, |sym| sym.is_used))
        }
        _ => true,
    });
}
