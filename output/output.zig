const std = @import("std");

fn salam() usize {
    std.debug.print("{s}\n", .{"Salam"});
    return 1;
}

fn artir(c: *usize) usize {
    c.* = (c.* + 1);
    if ((c.* == 6)) {
    return c.*;
}
else if ((c.* == 7)) {
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
   const a: usize = salam();
   var b: usize = 5;
   std.debug.print("{}\n", .{a});
   for (1..1000) |i| {
    std.debug.print("{}\n", .{i});
}
   const c: usize = artir(&b);
   std.debug.print("{}\n", .{c});
   const Adam = struct {
    ad: []const u8,
    soyad: []const u8,
    yas: usize,

    pub fn qeydi(self: @This()) void {
        std.debug.print("Mən {s} \n", .{ self.ad });
    }
};
   const adam: Adam = Adam{ .ad = "Prest", .soyad = "Guliyev", .yas = 17 };
   adam.qeydi();
   const Rengler = enum {
    Qirmizi,
    Yasil,
    Qara,
};
   const reng: Rengler = .Qirmizi;
   switch (reng) {
.Qirmizi => {
    std.debug.print("{s}\n", .{"Qirmizi"});
},
.Yasil => {
    std.debug.print("{s}\n", .{"Yasil"});
},
.Qara => {
    std.debug.print("{s}\n", .{"Qara"});
}
}
   var buf_ad: [100]u8 = undefined;
const ad: []u8 = try input("Ad\u{131}n\u{131}z\u{131} girin.", &buf_ad);
   const yas: usize = try std.fmt.parseInt(usize, (blk: {
                var buf_temp: [100]u8 = undefined;
                break :blk try input("Ya\u{15f}\u{131}n\u{131}z\u{131} girin.", &buf_temp);
            }), 10);
   std.debug.print("Mən {s} {} \n", .{ ad, yas });
}
