const std = @import("std");

fn salam(ad: []const u8) usize {
    std.debug.print("Salam, {s}! Xoş gəldin.\n", .{ ad });
    return 1;
}

fn artir(c: *usize) usize {
    c.* = (c.* + 1);
    if ((c.* == 18)) {
    std.debug.print("{s}\n", .{"Art\u{131}q b\u{f6}y\u{fc}ks\u{259}n!"});
    return c.*;
}
else if ((c.* == 17)) {
    std.debug.print("{s}\n", .{"Bir ya\u{15f}\u{131}n qal\u{131}b b\u{f6}y\u{fc}m\u{259}y\u{259}."});
    return c.*;
}
    else {
    return c.*;
}
}



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
   std.debug.print("{}\n", .{1});
   const Adam = struct {
    ad: []const u8,
    soyad: []const u8,
    yas: usize,

    pub fn qeydi(self: @This()) void {
        std.debug.print("Mən {s} {s}, {} yaşım var.\n", .{ self.ad, self.soyad, self.yas });
    }
};
   var buf_ad: [100]u8 = undefined;
const ad: []u8 = try input("Ad\u{131}n\u{131}z\u{131} daxil edin:", &buf_ad);
   var buf_soyad: [100]u8 = undefined;
const soyad: []u8 = try input("Soyad\u{131}n\u{131}z\u{131} daxil edin:", &buf_soyad);
   const yas: usize = try std.fmt.parseInt(usize, (blk: {
                var buf_temp: [100]u8 = undefined;
                break :blk try input("Ya\u{15f}\u{131}n\u{131}z\u{131} daxil edin:", &buf_temp);
            }), 10);
}
