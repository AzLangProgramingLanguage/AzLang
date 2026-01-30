const std = @import("std");

pub fn to_string(
    comptime T: type,
    allocator: std.mem.Allocator,
    value: T,
) ![]u8 {
    return switch (@typeInfo(T)) {
        .int, .float => std.fmt.allocPrint(allocator, "{d}", .{value}),
        .bool => std.fmt.allocPrint(allocator, "{s}", .{if (value) "true" else "false"}),
        .array => |a| if (a.child == u8)
            std.fmt.allocPrint(allocator, "{s}", .{value})
        else
            arrayVisit(T, allocator, value),
        .pointer => |p| switch (@typeInfo(p.child)) {
            .array => |a| if (a.child == u8)
                std.fmt.allocPrint(allocator, "{s}", .{value})
            else
                arrayVisit(T, allocator, value),
            else => arrayVisit(T, allocator, value),
        },
        else => std.fmt.allocPrint(allocator, "<unsupported>", .{}),
    };
}

fn arrayVisit(
    comptime T: type,
    allocator: std.mem.Allocator,
    value: T,
) ![]u8 {
    var list = std.array_list.Aligned(u8, null).empty;
    defer list.deinit(allocator);

    try list.append(allocator, '[');
    for (value, 0..) |val, i| {
        if (i != 0)
            try list.appendSlice(allocator, ",");
        const s = try to_string(@TypeOf(val), allocator, val);
        defer allocator.free(s);
        try list.appendSlice(allocator, s);
    }
    try list.append(allocator, ']');

    return list.toOwnedSlice(allocator);
}

test "to_string: tam ədədlər" {
    const allocator = std.testing.allocator;
    const result = try to_string(u8, allocator, 42);
    defer allocator.free(result);
    try std.testing.expectEqualStrings("42", result);
}

test "to_string: stringlər olduğu kimi" {
    const allocator = std.testing.allocator;
    const result = try to_string(@TypeOf("Salam"), allocator, "Salam");
    defer allocator.free(result);
    try std.testing.expectEqualStrings("Salam", result);
}

test "to_string: array testi" {
    const allocator = std.testing.allocator;
    const result = try to_string([3]i32, allocator, [_]i32{ 1, 2, 3 });
    defer allocator.free(result);
    try std.testing.expectEqualStrings("[1,2,3]", result);
}

