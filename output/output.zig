const std = @import("std");



pub fn input(prompt: []const u8, buf: []u8) ![]u8 {
    const stdin = std.io.getStdIn().reader();
    std.debug.print("{s} ", .{prompt});
    if (try stdin.readUntilDelimiterOrEof(buf, '\n')) |line| {
        return line;
    } else {
        return error.EmptyInput;
    }
}


pub fn main() !void {
    const eded: usize = try std.fmt.parseInt(usize, (blk: {
                var buf_temp: [100]u8 = undefined;
                break :blk try input("\u{259}d\u{259}d girin: ", &buf_temp);
            }), 10);
    var buf_isare: [100]u8 = undefined;
const isare: []u8 = try input("i\u{15f}ar\u{259} girin: ", &buf_isare);
    const eded2: usize = try std.fmt.parseInt(usize, (blk: {
                var buf_temp: [100]u8 = undefined;
                break :blk try input("\u{259}d\u{259}d2 girin: ", &buf_temp);
            }), 10);
    var qiymet: usize = 0;
    if (std.mem.eql(u8, isare, "+")) {
    qiymet = (eded + eded2);
}
else if (std.mem.eql(u8, isare, "-")) {
    qiymet = (eded - eded2);
}
    else if (std.mem.eql(u8, isare, "*")) {
    qiymet = (eded * eded2);
}
    else if (std.mem.eql(u8, isare, "/")) {
    qiymet = (eded / eded2);
}
    else {
    std.debug.print("{s}\n", .{"yanl\u{131}\u{15f} i\u{15f}ar\u{259}"});
}
    std.debug.print(" Cavab: {} \n", .{ qiymet });
}
