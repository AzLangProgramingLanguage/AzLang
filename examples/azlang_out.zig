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

pub fn str_trim(self: azlangYazi, allocator: std.mem.Allocator, mut: bool) !azlangYazi {
    const slice = switch (self) {
        .Mut => self.Mut,
        .Const => self.Const,
    };

    const trimmed = std.mem.trim(u8, slice, " ");
    if (mut) {
        const buf = try allocator.alloc(u8, trimmed.len);
        return azlangYazi{ .Mut = buf };
    } else {
        return azlangYazi{ .Const = trimmed };
    }
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
    const T = comptime @TypeOf(value);
    var str_val: []u8 = undefined;

    if (T == azlangEded) {
        switch (value) {
            .natural => |n| {
                str_val = try std.fmt.allocPrint(allocator, "{}", .{n});
            },
            .integer => |i| {
                str_val = try std.fmt.allocPrint(allocator, "{}", .{i});
            },
            .float => |f| {
                str_val = try std.fmt.allocPrint(allocator, "{d}", .{f});
            },
        }
    } else {
        str_val = try std.fmt.allocPrint(allocator, "{}", .{value});
    }

    if (mut) {
        return azlangYazi{ .Mut = str_val };
    } else {
        return azlangYazi{ .Const = str_val };
    }
}

const azlangYazi = union(enum) {
    Const: []const u8,
    Mut: []u8,

    pub fn Yeni(
        self: @This(),
    ) azlangYazi {
        return self;
    }
    pub fn azlangboyut(self: @This(), allocator: std.mem.Allocator) !azlangYazi {
        return try str_uppercase(allocator, self, false);
    }
    pub fn azlangkicilt(self: @This(), allocator: std.mem.Allocator) !azlangYazi {
        return try str_lowercase(allocator, self, false);
    }
    pub fn azlangters(self: @This(), allocator: std.mem.Allocator) !azlangYazi {
        return try str_reverse(allocator, self, false);
    }
    pub fn azlangqirx(self: @This(), allocator: std.mem.Allocator) !azlangYazi {
        return try str_trim(self, allocator, false);
    }
    pub fn free(self: azlangYazi, allocator: std.mem.Allocator) void {
        switch (self) {
            .Mut => |buf| allocator.free(buf),
            .Const => |buf| allocator.free(buf),
        }
    }

    pub fn TipVer(
        self: @This(),
    ) []const u8 {
        _ = self;
        return "Yaz\u{131}";
    }
    pub fn uzunluq(
        self: @This(),
    ) azlangEded {
        switch (self) {
            .Const => return azlangEded.Yeni(azlangEded{ .natural = self.Const.len }),
            .Mut => return azlangEded.Yeni(azlangEded{ .natural = self.Mut.len }),
        }
    }
};

const azlangSiyahi = union(enum) {
    Const: []const usize,
    Mut: []usize,

    pub fn uzunluq(
        self: @This(),
    ) azlangEded {
        switch (self) {
            .Const => return azlangEded.Yeni(azlangEded{ .natural = self.Const.len }),
            .Mut => return azlangEded.Yeni(azlangEded{ .natural = self.Mut.len }),
        }
    }
    pub fn TipVer(
        self: @This(),
    ) []const u8 {
        _ = self;
        return "Siyahi";
    }
};
fn azlang_add(a: azlangEded, b: azlangEded) azlangEded {
    const af = a.toFloat();
    const bf = b.toFloat();
    const res = af + bf;
    if (@floor(res) == res) {
        if (res >= 0) {
            return azlangEded{ .natural = @intCast(res) };
        } else {
            return azlangEded{ .integer = @intCast(res) };
        }
    } else {
        return azlangEded{ .float = res };
    }
}

fn azlang_sub(a: azlangEded, b: azlangEded) azlangEded {
    const ai = a.toInteger();
    const bi = b.toInteger();
    const res = ai - bi;
    if (res >= 0) {
        return azlangEded{ .natural = @intCast(res) };
    } else return azlangEded{ .integer = res };
}

fn azlang_mul(a: azlangEded, b: azlangEded) azlangEded {
    const ai = a.toInteger();
    const bi = b.toInteger();
    return azlangEded{ .integer = ai * bi };
}

const azlangEded = union(enum) {
    natural: usize,
    integer: isize,
    float: f64,
    pub fn toInteger(self: azlangEded) isize {
        return switch (self) {
            .natural => |n| @intCast(n),
            .integer => |i| i,
            .float => |f| @intFromFloat(f), // truncation ola bilər
        };
    }

    pub fn toFloat(self: azlangEded) f64 {
        return switch (self) {
            .natural => |n| @floatFromInt(n),
            .integer => |i| @floatFromInt(i),
            .float => |f| f,
        };
    }

    pub fn Yeni(
        self: @This(),
    ) azlangEded {
        return self;
    }
    pub fn azlangyaziya_cevir(self: @This(), allocator: std.mem.Allocator) !azlangYazi {
        return try convert_string(allocator, self, true);
    }
    pub fn TipVer(
        self: @This(),
    ) []const u8 {
        _ = self;
        return "\u{18f}d\u{259}d";
    }
};

fn faktorial(x: azlangEded) azlangEded {
    if (x.toInteger() == 0) {
        return azlangEded{ .integer = 1 };
    } else {
        const one = azlangEded{ .natural = 1 };
        const sub = azlang_sub(x, one);
        return azlang_mul(x, faktorial(sub));
    }
}
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const result = faktorial(azlangEded{ .natural = 5 });
    const s = try result.azlangyaziya_cevir(allocator);
    defer s.free(allocator);
    std.debug.print("Faktorial(5) = {s}\n", .{s.Mut});
}
