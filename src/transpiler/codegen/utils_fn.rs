use crate::context::TranspileContext;

pub fn generate_util_functions(ctx: &TranspileContext) -> String {
    let mut code = String::new();
    if ctx.used_input_fn {
        code.push_str(
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
    if ctx.is_find_method {
        code.push_str(
            r#"
fn find_index(list: []const []const u8, value: []const u8) i64 {
    for (list, 0..) |item, i| {
        if (std.mem.eql(u8, item, value)) return @intCast(i);
    }
    return -1;
}
"#,
        );
    }

    if ctx.used_sum_fn {
        code.push_str(
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
        code.push_str(
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
        code.push_str(r#"
pub fn splitAuto(allocator: std.mem.Allocator, input: []const u8, delimiter: u8) ![]const []const u8 {
    var list = std.ArrayList([]const u8).init(allocator);
    var iter = std.mem.splitScalar(u8, input, delimiter);
    while (iter.next()) |part| {
        try list.append(part);
    }
    return try list.toOwnedSlice();
}
"#);
    }
    if ctx.used_split_alloc_fn {
        code.push_str(r#"
pub fn splitNAlloc(allocator: std.mem.Allocator, input: []const u8, delimiter: u8) !std.ArrayList([]const u8) {
    var list = std.ArrayList([]const u8).init(allocator);
    var iter = std.mem.splitScalar(u8, input, delimiter);
    while (iter.next()) |part| {
        try list.append(part);
    }
    return list;
}
"#);
    }

    code
}
