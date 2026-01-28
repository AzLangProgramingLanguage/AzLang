const std = @import("std");
fn isString(comptime T: type) bool {
    const info = @typeInfo(T);
    if (T == []u8 or T == []const u8) return true;
    if (info == .pointer) {
        const child = info.pointer.child;
        const child_info = @typeInfo(child);
        if (child_info == .array and child_info.array.child == u8) return true;
    }
    return false;
}
pub fn to_string(allocator: std.mem.Allocator, value: anytype) ![]u8 {
    const T = @TypeOf(value);
    const info = @typeInfo(T);

    if (comptime isString(T)) {
        return std.fmt.allocPrint(allocator, "{s}", .{value});
    }

    if (info == .array or info == .pointer) {
        var list = try std.ArrayList(u8).initCapacity(allocator, value.len * 2);
        errdefer list.deinit(allocator);

        try list.append(allocator, '[');
        for (value, 0..) |item, i| {
            const item_str = try to_string(allocator, item);
            defer allocator.free(item_str);

            try list.appendSlice(item_str);
            if (i < value.len - 1) {
                try list.appendSlice(", ");
            }
        }
        try list.append(allocator, ']');
        return list.toOwnedSlice();
    }

    return std.fmt.allocPrint(allocator, "{any}", .{value});
}
test "to_string: tam ədədlər" {
    const allocator = std.testing.allocator;

    const result = try to_string(allocator, 42);
    defer allocator.free(result);

    try std.testing.expectEqualStrings("42", result);
}
test "to_string: stringlər olduğu kimi" {
    const allocator = std.testing.allocator;

    const result = try to_string(allocator, "Salam");
    defer allocator.free(result);
    try std.testing.expectEqualStrings("Salam", result);
}
test "to_string: array testi" {
    const allocator = std.testing.allocator;
    const result = try to_string(allocator, [_]i32{ 1, 2, 3 });
    defer allocator.free(result);

    try std.testing.expectEqualStrings("[1, 2, 3]", result);
}
