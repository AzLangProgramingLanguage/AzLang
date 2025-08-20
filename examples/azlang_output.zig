const std = @import("std");

pub fn str_uppercase(allocator: std.mem.Allocator, self: azlangYazi, mut: bool) !azlangYazi {
    const slice = switch (self) {
        .Mut => self.Mut,
        .Const => self.Const,
    };
    const output = try allocator.alloc(u8, slice.len);
    _ = std.ascii.upperString(output, slice);
    return if (mut)
        azlangYazi{ .Mut = output }
    else
        azlangYazi{ .Const = output };
}
pub fn str_reverse(
    allocator: std.mem.Allocator,
    self: azlangYazi,
    mut: bool,
) !azlangYazi {
    const slice = switch (self) {
        .Mut => self.Mut,
        .Const => self.Const,
    };

    const len = slice.len;
    const output = try allocator.alloc(u8, len);
    std.mem.copyForwards(u8, output, slice);
    std.mem.reverse(u8, output);

    return if (mut)
        azlangYazi{ .Mut = output }
    else
        azlangYazi{ .Const = output };
}

pub fn str_lowercase(allocator: std.mem.Allocator, self: azlangYazi, mut: bool) !azlangYazi {
    const slice = switch (self) {
        .Mut => self.Mut,
        .Const => self.Const,
    };
    const output = try allocator.alloc(u8, slice.len);
    _ = std.ascii.lowerString(output, slice);
    return if (mut)
        azlangYazi{ .Mut = output }
    else
        azlangYazi{ .Const = output };
}
pub fn convert_string(allocator: std.mem.Allocator, value: anytype, mut: bool) !azlangYazi {
    const str_val = try std.fmt.allocPrint(allocator, "{}", .{value});
    if (mut) {
        return azlangYazi{ .Mut = str_val };
    } else {
        return azlangYazi{ .Const = str_val };
    }
}

const azlangYazi = union(enum) {
    Const: []const u8,
    Mut: []u8,

    pub fn azlangboyut(self: @This(), allocator: std.mem.Allocator) ![]const u8 {
        switch (self) {
            .Const => return try str_uppercase(allocator, self.Const, false),
            .Mut => return try str_uppercase(allocator, self.Mut, false),
        }
    }
    pub fn azlangkicilt(self: @This(), allocator: std.mem.Allocator) ![]const u8 {
        switch (self) {
            .Const => return try str_lowercase(allocator, self.Const, false),
            .Mut => return try str_lowercase(allocator, self.Mut, false),
        }
    }
    pub fn azlangyaziters(self: @This(), allocator: std.mem.Allocator) !azlangYazi {
        switch (self) {
            .Const => return try str_reverse(allocator, self.Const, false),
            .Mut => return try str_reverse(allocator, self.Mut, false),
        }
    }
    pub fn TipVer(
        self: @This(),
    ) ![]const u8 {
        _ = self;
        return "Yaz\u{131}";
    }
    pub fn uzunluq(
        self: @This(),
    ) !azlangEded {
        switch (self) {
            .Const => return self.Const.len,
            .Mut => return self.Mut.len,
        }
    }
};

const azlangEded = struct {
    deyer: isize,

    pub fn Yeni(x: isize) @This() {
        return .{ .deyer = x };
    }

    pub fn TipVer(self: @This()) []const u8 {
        _ = self;
        return "\u{18f}d\u{259}d";
    }
};

const azlangSiyahi = union(enum) {
    Const: []const usize,
    Mut: []usize,

    pub fn uzunluq(
        self: @This(),
    ) !azlangEded {
        switch (self) {
            .Const => return self.Const.len,
            .Mut => return self.Mut.len,
        }
    }
    pub fn TipVer(
        self: @This(),
    ) ![]const u8 {
        _ = self;
        return "Siyahi";
    }
};

fn factorial(n: azlangEded) azlangEded {
    if ((n.deyer <= azlangEded.Yeni(1))) {
        return azlangEded{ .deyer = 1 };
    } else {
        return factorial((n.deyer - azlangEded.Yeni(1)));
    }
}

pub fn main() !void {
    const eded: azlangEded = azlangEded{ .deyer = 5 };
    std.debug.print("{!}\n", .{eded.deyer});
    const vaxt1: azlangEded = azlangEded{ .deyer = @intCast(std.time.milliTimestamp()) };
    const vaxt2: azlangEded = azlangEded{ .deyer = @intCast(std.time.milliTimestamp()) };
    std.debug.print("Vaxt {any}ms\n", .{(vaxt2.deyer - vaxt1.deyer)});
}
