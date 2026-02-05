use parser::{
    ast::{Expr, TemplateChunk},
    shared_ast::Type,
};

use crate::{
    TranspileContext,
    builtin::print::transpile_print,
    helper::{get_expr_type, get_format_str_from_type, is_primite_value},
    transpile::transpile_expr,
};

pub fn transpile_input<'a>(expr: Expr<'a>, ctx: &mut TranspileContext<'a>) -> String {
    let mut print_value = String::from("{");
    print_value.push_str(&transpile_print(expr, ctx));
    print_value
}
