const std = @import("std");
const c = @cImport({
    @cInclude("stdio.h");
});

pub fn input(allocator: std.mem.Allocator, prompt: []const u8) ![]u8 {
    std.debug.print("{s}", .{prompt});
    var buffer: [1024]u8 = undefined;
    const result = c.scanf("%1023s", &buffer);
    if (result == -1) return error.InputFailed;
    const len = std.mem.indexOfScalar(u8, &buffer, 0) orelse 1024;
    const input_slice = buffer[0..len];
    const zig_slice = try allocator.dupe(u8, input_slice);
    return zig_slice;
}
