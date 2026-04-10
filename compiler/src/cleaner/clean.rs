use parser::ast::{Expr, Program, Statement};
use validator::Validator;

pub fn clean_ast<'a>(program: &mut Program, ctx: &Validator) {
    program.expressions.retain(|statement| match statement {
        Statement::Decl { name, .. } => {
            let name_str: &str = name.as_ref();
            ctx.global_variables.get(name_str).unwrap().is_used
        }
        _ => true,
    });
}
