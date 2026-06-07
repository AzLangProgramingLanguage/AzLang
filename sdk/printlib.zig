const std = @import("std");

const Value = union(enum) {
    String: []const u8,
    Number: u8,
};

fn printValue(a: Value) void {
    switch (a) {
        .String => |str| std.debug.print("{s}\n", .{str}),
        .Number => |num| std.debug.print("{d}\n", .{num}),
    }
}

test "hello" {
    const v1 = Value{ .Number = 5 };
    const v2 = Value{ .String = "Salam" };

    printValue(v1);
    printValue(v2);

    try std.testing.expectEqual(@as(u8, 5), 5);
}
