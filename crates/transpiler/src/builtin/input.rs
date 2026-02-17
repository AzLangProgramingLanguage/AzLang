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
    let transpiled_expr = transpile_expr(expr, ctx);
    ctx.needs_allocator = true;
    if !ctx.imports.contains("const std = @import(\"std\");") {
        ctx.imports
            .insert("const input = @import(\"./dependencies/input.zig\").input;\n".to_string());
    }
    format!("try input(allocator,{transpiled_expr})")
}
