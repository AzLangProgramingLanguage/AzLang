const std = @import("std");




pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    var a = try std.ArrayList(usize).initCapacity(allocator, 4);
try a.appendSlice(&[_]usize{ 1, 2, 3, 4 });
    std.debug.print("{}\n", .{a.items[0]});
    a.deinit();
}
