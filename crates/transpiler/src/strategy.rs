use parser::{ast::Expr, shared_ast::Type};

pub struct VariableDecl;
impl VariableDecl {
    pub fn transpile(name: String, typ: &Type, is_mutable: bool, value: Expr) -> String {
        match (typ, value) {
            (&Type::String, Expr::String(s)) => todo!(),
            (_, _) => todo!(),
        }
    }
}
fn transpile_string_primitive(name: String, is_mutable: bool) -> String {
    format!()
}
