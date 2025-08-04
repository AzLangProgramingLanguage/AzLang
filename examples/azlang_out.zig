const std = @import("std");

const StringLike = union(enum) {
    Const: []const u8,
    Mut: []u8,

    pub fn asConst(self: StringLike) []const u8 {
        return switch (self) {
            .Const => |v| v,
            .Mut => |v| v,
        };
    }
    pub fn toUpperCase(self: StringLike, allocator: std.mem.Allocator) !StringLike {
        return switch (self) {
            .Const => StringLike{ .Const = try str_uppercase(allocator, self.Const) },
            .Mut => StringLike{ .Mut = try str_uppercase(allocator, self.Mut) },
        };
    }
};

pub fn str_uppercase(allocator: std.mem.Allocator, self: []const u8) ![]u8 {
    const output = try allocator.alloc(u8, self.len);
    return std.ascii.upperString(output, self);
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const immutable: []const u8 = "salam";
    const mutable: []u8 = try allocator.dupe(u8, "dunya");

    const upper1 = (try StringLike.toUpperCase(.{ .Const = immutable }, allocator)).Const;
    const upper2 = (try StringLike.toUpperCase(.{ .Mut = mutable }, allocator)).Mut;

    defer allocator.free(upper1);
    defer allocator.free(upper2);
    defer allocator.free(mutable);

    std.debug.print("Upper1: {s}\n", .{upper1});
    std.debug.print("Upper2: {s}\n", .{upper2});
}
