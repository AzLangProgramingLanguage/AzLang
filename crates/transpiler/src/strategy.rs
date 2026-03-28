use parser::{ast::Expr, shared_ast::Type};

use crate::{TranspileContext, helper::map_type, transpile::transpile_expr};

pub struct VariableDecl;
impl VariableDecl {
    pub fn transpile<'a>(
        name: String,
        typ: &Type,
        is_mutable: bool,
        value: Expr,
        ctx: &mut TranspileContext,
    ) -> String {
        match (&typ, &value) {
            (&Type::LiteralString, Expr::String(s)) => {
                transpile_string_primitive(name, is_mutable, s)
            }
            (&Type::Integer, Expr::Number(n)) => transpile_number_primitive(name, is_mutable, n),
            (&Type::String, expr) => transpile_string_nonprimitive(name, is_mutable, expr, ctx),
            (&Type::Array(t), Expr::List(e)) => {
                transpile_array_primitive(name, is_mutable, e, t, ctx)
            }

            (_, _) => todo!(),
        }
    }
}
fn transpile_array_primitive(
    name: String,
    is_mutable: bool,
    value: &Vec<Expr>,
    typ: &Box<Type>,
    ctx: &mut TranspileContext,
) -> String {
    let symbol = is_mutable_symbol(is_mutable);
    format!(
        "{symbol} {name}= {}",
        transpile_expr(Expr::List(value.clone()), ctx)
    )
}
fn transpile_string_nonprimitive<'a>(
    name: String,
    is_mutable: bool,
    value: &Expr,
    ctx: &mut TranspileContext<'a>,
) -> String {
    format!(
        "{} {name}: []u8 = {}",
        is_mutable_symbol(is_mutable),
        transpile_expr(value.clone(), ctx)
    )
}
fn transpile_string_primitive(name: String, is_mutable: bool, value: &String) -> String {
    format!(
        "{} {name}: []const u8 = \"{value}\" ",
        is_mutable_symbol(is_mutable)
    )
}
fn transpile_number_primitive(name: String, is_mutable: bool, value: &i64) -> String {
    format!("{} {name}: i64 = {value} ", is_mutable_symbol(is_mutable))
}

fn is_mutable_symbol(is_mutable: bool) -> &'static str {
    if is_mutable { "var" } else { "const" }
}
