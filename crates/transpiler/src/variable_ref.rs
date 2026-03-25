// use parser::{ast::Expr, shared_ast::Type};
//
// pub struct VariableRef;
// impl VariableRef {
//     pub fn transpile(name: String, typ: &Type, is_mutable: bool, value: Expr) -> String {
//         match (typ, value) {
//             (&Type::LiteralString, Expr::String(s)) => {
//                 transpile_string_primitive(name, is_mutable, s)
//             }
//             (&Type::Integer, Expr::Number(n)) => transpile_number_primitive(name, is_mutable, n),
//             (_, _) => todo!(),
//         }
//     }
// }
// fn transpile_string_primitive(name: String, is_mutable: bool, value: String) -> String {
//     format!(
//         "{} {name}: []const u8 = \"{value}\" ",
//         is_mutable_symbol(is_mutable)
//     )
// }
// fn transpile_number_primitive(name: String, is_mutable: bool, value: i64) -> String {
//     format!("{} {name}: i64 = {value} ", is_mutable_symbol(is_mutable))
// }
//
// fn is_mutable_symbol(is_mutable: bool) -> &'static str {
//     if is_mutable { "var" } else { "const" }
// }
