use super::expr::transpile_expr;
use crate::{
    context::{Parameter, TranspileContext},
    function::transpile_function_def,
    parser::{Expr, Program},
};

pub fn generate_main_fn(
    program: &Program,
    ctx: &mut TranspileContext,
    message: &dyn Fn(&str),
) -> Result<String, String> {
    let mut top_level_defs = String::new(); // Funksiya tərifləri buraya yazılacaq
    let mut main_body = String::new(); // Yalnız çağırışlar, `main()` içi

    for expr in &program.expressions {
        match expr {
            Expr::FunctionDef {
                name,
                params,
                body,
                return_type,
            } => {
                let def = transpile_function_def(name, params, body, return_type.clone(), ctx)?;
                top_level_defs.push_str(&def);
                top_level_defs.push_str("\n\n");
            }

            other_expr => {
                let line = transpile_expr(other_expr, ctx)?;
                let line = if line.trim_end().ends_with(';') {
                    line
                } else {
                    format!("{}", line)
                };
                main_body.push_str(&line);
                main_body.push_str("\n");
            }
        }
    }

    let mut zig_code = String::new();

    // 📦 Zig modüllerini (`@import`) əlavə et
    if !ctx.imports.contains("const std = @import(\"std\");") {
        zig_code.push_str("const std = @import(\"std\");\n");
    }

    for import in &ctx.imports {
        if import != "const std = @import(\"std\");" {
            zig_code.push_str(import);
            zig_code.push('\n');
        }
    }

    // ✨ Əlavə funksiya tərifləri
    zig_code.push_str(&top_level_defs);

    // Əlavə util funksiyalar (input, sum və s.)
    if ctx.used_input_fn {
        zig_code.push_str(
            r#"

pub fn input(prompt: []const u8, buf: []u8) ![]u8 {
    const stdin = std.io.getStdIn().reader();
    std.debug.print("{s} ", .{prompt});
    if (try stdin.readUntilDelimiterOrEof(buf, '\n')) |line| {
        return line;
    } else {
        return error.EmptyInput;
    }
}
"#,
        );
    }

    if ctx.used_sum_fn {
        zig_code.push_str(
            r#"
pub fn sum(comptime T: type, list: []const T) T {
    var total: T = 0;
    for (list) |item| {
        total += item;
    }
    return total;
}
"#,
        );
    }

    if ctx.used_split_n_fn {
        zig_code.push_str(
            r#"
const MAX_PARTS = 32;

pub const SplitResult = struct {
    parts: [MAX_PARTS][]const u8,
    len: usize,
};

pub fn splitN(input: []const u8, delimiter: u8, count: usize) SplitResult {
    var parts: [MAX_PARTS][]const u8 = undefined;
    var i: usize = 0;
    var iter = std.mem.splitScalar(u8, input, delimiter);
    while (iter.next()) |part| {
        if (i >= count or i >= MAX_PARTS) break;
        parts[i] = part;
        i += 1;
    }
    return SplitResult{ .parts = parts, .len = i };
}
"#,
        );
    }

    if ctx.used_split_auto_fn {
        ctx.add_import("const std = @import(\"std\");");
        zig_code.push_str(
            r#"
pub fn splitAuto(allocator: std.mem.Allocator, input: []const u8, delimiter: u8) ![]const []const u8 {
    var list = std.ArrayList([]const u8).init(allocator);
    var iter = std.mem.splitScalar(u8, input, delimiter);
    while (iter.next()) |part| {
        try list.append(part);
    }
    return try list.toOwnedSlice();
}
"#,
        );
    }

    // ✨ Zig'de main funksiyası
    zig_code.push_str("\npub fn main() !void {\n");

    if ctx.needs_allocator {
        zig_code.push_str("    var gpa = std.heap.GeneralPurposeAllocator(.{}){};\n");
        zig_code.push_str("    const allocator = gpa.allocator();\n");
    }

    zig_code.push_str(&main_body);

    for var_name in &ctx.cleanup_statements {
        zig_code.push_str("    ");
        zig_code.push_str(var_name);
        zig_code.push('\n');
    }

    zig_code.push_str("}\n");

    Ok(zig_code)
}
