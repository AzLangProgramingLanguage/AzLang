const std = @import("std");

pub fn main() !void {
const Adam = struct {
    ad: []const u8,
    age: usize,
};
const adam: Adam = Adam{ .ad = "S\u{259}buhi", .age = 19 };
std.debug.print("{s}\n", .{adam.ad});
}
