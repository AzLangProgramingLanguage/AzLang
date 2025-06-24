const std = @import("std");

fn artir(b: *usize) void {
    b.* = (b.* + 1);
}




pub fn main() !void {
    var a: usize = 4;
    artir(&a);
    std.debug.print("{}\n", .{a});
}
