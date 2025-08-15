pub const BUILTIN_FUNCTIONS: &str = r#"

pub fn str_uppercase(allocator: std.mem.Allocator, self: azlangYazi) !azlangYazi {
    switch (self) {
        .Const => {
            const output = try allocator.alloc(u8, self.Const.len);
            std.ascii.upperString(output, self.Const);
            return azlangYazi{ .Const = output };
        },
        .Mut => {
            const output = try allocator.alloc(u8, self.Mut.len);
            std.ascii.upperString(output, self.Mut);
            return azlangYazi{ .Mut = output };
        },
    }
}

pub fn str_reverse(allocator: std.mem.Allocator, self: azlangYazi) ![]u8 {
    switch (self) {
        .Const => {
            const len = self.Const.len;
            const output = try allocator.alloc(u8, len);
            std.mem.copyForwards(u8, output, self.Const);
            std.mem.reverse(u8, output);
            return output;
        },
        .Mut => {
            const len = self.Mut.len;
            const output = try allocator.alloc(u8, len);
            std.mem.copyForwards(u8, output, self.Mut);
            std.mem.reverse(u8, output);
            return output;
        },
    }
}

pub fn str_lowercase(allocator: std.mem.Allocator, self: azlangYazi) !azlangYazi {
    switch (self) {
        .Const => {
            const output = try allocator.alloc(u8, self.Const.len);
            std.ascii.lowerString(output, self.Const);
            return azlangYazi{ .Const = output };
        },
        .Mut => {
            const output = try allocator.alloc(u8, self.Mut.len);
            std.ascii.lowerString(output, self.Mut);
            return azlangYazi{ .Mut = output };
        },
    }
 
}
       pub fn convert_string(allocator: std.mem.Allocator, value: anytype) !azlangYazi {
    const str_val = try std.fmt.allocPrint(allocator, "{}", .{value});
    return azlangYazi{ .Mut = str_val };
}
"#;
