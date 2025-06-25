const std = @import("std");

fn artir(a: *usize) void {
    a.* = (a.* + 1);
}




pub fn main() !void {
    var a: usize = 0;
    artir(&a);
    std.debug.print("{}\n", .{a});
}
