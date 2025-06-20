use crate::{codegen, context::TranspileContext, parser::Program};

pub fn generate_main_fn(
    program: &Program,
    ctx: &mut TranspileContext,
    _message: &dyn Fn(&str),
) -> Result<String, String> {
    let imports = codegen::prelude::generate_imports(ctx);
    let defs = codegen::top_level::generate_top_level_defs(program, ctx)?;
    let main_body = codegen::main_body::generate_main_body(program, ctx)?;

    let utils = codegen::utils_fn::generate_util_functions(ctx);

    let allocator = if ctx.needs_allocator {
        "    var gpa = std.heap.GeneralPurposeAllocator(.{}){};\n    const allocator = gpa.allocator();\n"
    } else {
        ""
    };
    let cleanup: String = ctx
        .cleanup_statements
        .iter()
        .map(|s| format!("    {}\n", s))
        .collect();

    Ok(format!(
        r#"{imports}
{defs}
{utils}

pub fn main() !void {{
{allocator}{main_body}{cleanup}}}
"#,
    ))
}
