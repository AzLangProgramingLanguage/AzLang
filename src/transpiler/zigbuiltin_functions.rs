pub const BUILTIN_FUNCTIONS: &str = r#"



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
"#;
