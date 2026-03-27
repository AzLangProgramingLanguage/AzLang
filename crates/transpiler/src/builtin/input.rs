use parser::ast::{Expr, TemplateChunk};

use crate::{
    TranspileContext,
    helper::{get_expr_type, get_format_str_from_type, is_primite_value},
    transpile::transpile_expr,
};

pub fn transpile_input<'a>(expr: Expr, ctx: &mut TranspileContext<'a>) -> String {
    let transpiled_expr = transpile_expr(expr, ctx);
    ctx.needs_allocator = true;
    if !ctx.imports.contains("const std = @import(\"std\");") {
        ctx.imports.insert(
            "const input_alloc = @import(\"./dependencies/input.zig\").input_alloc;\n".to_string(),
        );
    }
    format!("try input_alloc(allocator)")
}
