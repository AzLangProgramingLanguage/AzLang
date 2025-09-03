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

const azlangEded = union(enum) {
    natural: usize,
    integer: isize,
    float: f64,

    pub fn Yeni(
        self: @This(),
    ) azlangEded {
        return self;
    }
    pub fn azlangyaziya_cevir(self: @This(), allocator: std.mem.Allocator) !azlangYazi {
        return try convert_string(allocator, self, false);
    }
    pub fn TipVer(
        self: @This(),
    ) []const u8 {
        _ = self;
        return "\u{18f}d\u{259}d";
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

pub fn max(list: []const azlangEded) !azlangEded {
    if (list.len == 0) {
        return error.EmptyList;
    }

    var max_value = list[0];
    for (list[1..]) |item| {
        if (compare(item, max_value) > 0) {
            max_value = item;
        }
    }
    return max_value;
}

fn compare(a: azlangEded, b: azlangEded) i2 {
    // hamısını f64 olaraq müqayisə edək
    const fa: f64 = switch (a) {
        .natural => |v| @floatFromInt(v),
        .integer => |v| @floatFromInt(v),
        .float => |v| v,
    };

    const fb: f64 = switch (b) {
        .natural => |v| @floatFromInt(v),
        .integer => |v| @floatFromInt(v),
        .float => |v| v,
    };

    if (fa > fb) return 1;
    if (fa < fb) return -1;
    return 0;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const a: azlangEded = azlangEded.Yeni(azlangEded{ .natural = 1 });
    const b: azlangEded = azlangEded.Yeni(azlangEded{ .natural = 5 });
    const c: azlangEded = azlangEded.Yeni(azlangEded{ .float = 1.4 });
    std.debug.print("{s}\n", .{"\u{18f}d\u{259}d n\u{fc}mun\u{259}l\u{259}ri:"});
    std.debug.print("{}\n", .{a.natural});
    std.debug.print("{}\n", .{b.natural});
    std.debug.print("{d}\n", .{c.float});
    std.debug.print("Cəmi: {any}\n", .{@as(f64, @floatFromInt(@as(isize, @intCast(a.natural)) + @as(isize, @intCast(b.natural)))) + c.float});
    std.debug.print("{}\n", .{(try (try (try azlangYazi.Yeni(azlangYazi{ .Const = "    salam dünya         " }).azlangboyut(allocator)).azlangters(allocator)).azlangqirx(allocator)).uzunluq().natural});
    const nums = [_]azlangEded{
        azlangEded{ .natural = 1 },
        azlangEded{ .integer = -4 },
        azlangEded{ .float = 11.4 },
        azlangEded{ .natural = 10 },
    };

    const result = try max(&nums);
    std.debug.print("Max: {any}\n", .{result.float});
}
