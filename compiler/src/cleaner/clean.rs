use parser::typed_ast::{CompiledProgram, TypedExpr};
use validator::validator_typed::ValidatorTypedContext;

pub fn clean_ast<'a>(program: &mut CompiledProgram<'a>, ctx: &ValidatorTypedContext<'a>) {
    program.expressions.retain(|expr| match expr {
        TypedExpr::Decl { name, .. } => {
            let name_str: &str = name.as_ref();
            ctx.scopes
                .iter()
                .any(|scope| scope.get(name_str).map_or(false, |sym| sym.is_used))
        }
        _ => true,
    });
}
