const std = @import("std");

pub export fn printValue_int(n: i64) void {
    std.debug.print("{d}\n", .{n});
}
pub export fn printValue_float(n: f64) void {
    std.debug.print("{d}\n", .{n});
}

pub export fn printValue_str(s: [*:0]const u8) void {
    std.debug.print("{s}\n", .{s});
}

pub export fn printValue_bool(b: u8) void {
    if (b != 0) {
        std.debug.print("doğru\n", .{});
    } else {
        std.debug.print("yanlış\n", .{});
    }
}

test "test" {
    const x: i32 = 42;
    const T = @TypeOf(x);

    std.debug.print("Type is: {}\n", .{T}); // Prints "Type is: i32"
}
