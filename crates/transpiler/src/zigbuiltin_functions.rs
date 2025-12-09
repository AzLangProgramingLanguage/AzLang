pub const BUILTIN_FUNCTIONS: &str = r#"

const ValueEnum = union(enum) {
   Array: azlangSiyahi,
   Number: azlangEded,
   String: azlangYazi,
};

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

fn azlang_add(a: azlangEded, b: azlangEded) azlangEded {
    const af = toFloat(a);
    const bf = toFloat(b);
    const res = af + bf;
    if (@floor(res) == res) {
        if (res >= 0) {
            return azlangEded{ .natural = @intFromFloat(res) };
        } else {
            return azlangEded{ .integer = @intFromFloat(res) };
        }
    } else {
        return azlangEded{ .float = res };
    }
}

   pub fn toFloat(self: azlangEded) f64 {
        return switch (self) {
            .natural => |n| @floatFromInt(n),
            .integer => |i| @floatFromInt(i),
            .float => |f| f,
        };
    }

 pub fn toInteger(self: azlangEded) isize {
        return switch (self) {
            .natural => |n| @intCast(n),
            .integer => |i| i,
            .float => |f| @intFromFloat(f), 
        };
    }


fn azlang_sub(a: azlangEded, b: azlangEded) azlangEded {
    const ai = toInteger(a);
    const bi = toInteger(b);
    const res = ai - bi;
    if (res >= 0) {
        return azlangEded{ .natural = @intCast(res) };
    } else return azlangEded{ .integer = res };
}

fn azlang_mul(a: azlangEded, b: azlangEded) azlangEded {
    const ai = toInteger(a);
    const bi = toInteger(b);
    return azlangEded{ .integer = ai * bi };
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


"#;
