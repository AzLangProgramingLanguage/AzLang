const std = @import("std");



const MAX_PARTS = 32;

pub const SplitResult = struct {
    parts: [MAX_PARTS][]const u8,
    len: usize,
};

pub fn splitN(input: []const u8, delimiter: u8, count: usize) SplitResult {
    var parts: [MAX_PARTS][]const u8 = undefined;
    var i: usize = 0;
    var iter = std.mem.splitScalar(u8, input, delimiter);
    while (iter.next()) |part| {
        if (i >= count or i >= MAX_PARTS) break;
        parts[i] = part;
        i += 1;
    }
    return SplitResult{ .parts = parts, .len = i };
}

pub fn splitNAlloc(allocator: std.mem.Allocator, input: []const u8, delimiter: u8) !std.ArrayList([]const u8) {
    var list = std.ArrayList([]const u8).init(allocator);
    var iter = std.mem.splitScalar(u8, input, delimiter);
    while (iter.next()) |part| {
        try list.append(part);
    }
    return list;
}


pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const a: []const u8 = "S,a,l,a,m";
    var b = try splitNAlloc(allocator, a, ',');
    const result_c = splitN(a, ',', 32);
const c = result_c.parts[0..result_c.len];
    std.debug.print("{s}\n", .{b.items[0]});
    std.debug.print("{s}\n", .{c[1]});
    const Adam = struct {
    ad: []const u8,
    soyad: []const u8,
    yas: usize,

    pub fn yasim(self: @This()) void {
        std.debug.print("{}\n", .{self.yas});
    }
};
    const adam: Adam = Adam{ .ad = "S\u{259}buhi", .soyad = "Sar\u{131}yev", .yas = 19 };
    adam.yasim();
    b.deinit();
}
