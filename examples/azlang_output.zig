const std = @import("std");

const String = struct {
    data: []const u8,

    pub fn fromSlice(slice: []const u8) String {
        return String{ .data = slice };
    }

    pub fn len(self: String) usize {
        return self.data.len;
    }

    pub fn mutable(self: String, allocator: std.mem.Allocator) !StringBuilder {
        var builder = try StringBuilder.init(allocator, self.len());
        try builder.push_str(self.data);
        return builder;
    }
    pub fn toUpperCase(self: String, allocator: std.mem.Allocator) !String {
        var builder = try self.mutable(allocator);
        builder.toUpperInPlace();
        return builder.toString();
    }
};

const StringBuilder = struct {
    allocator: std.mem.Allocator,
    buffer: []u8,
    len: usize,

    pub fn init(allocator: std.mem.Allocator, initial_capacity: usize) !StringBuilder {
        const buf = try allocator.alloc(u8, initial_capacity);
        return StringBuilder{
            .allocator = allocator,
            .buffer = buf,
            .len = 0,
        };
    }

    pub fn deinit(self: *StringBuilder) void {
        self.allocator.free(self.buffer);
        self.buffer = &[_]u8{};
        self.len = 0;
    }
    pub fn replaceInPlace(self: *StringBuilder, old: []const u8, new: []const u8) !void {
        var i: usize = 0;
        var new_buffer = try self.allocator.alloc(u8, self.buffer.len * 2);
        var new_len: usize = 0;

        while (i < self.len) {
            if (std.mem.startsWith(u8, self.buffer[i..self.len], old)) {
                std.mem.copyForwards(u8, new_buffer[new_len..], new);
                new_len += new.len;
                i += old.len;
            } else {
                new_buffer[new_len] = self.buffer[i];
                new_len += 1;
                i += 1;
            }
        }

        self.allocator.free(self.buffer);
        self.buffer = new_buffer;
        self.len = new_len;
    }

    pub fn ensureCapacity(self: *StringBuilder, new_len: usize) !void {
        if (new_len > self.buffer.len) {
            const new_capacity = @max(new_len, self.buffer.len * 2);
            const new_buf = try self.allocator.realloc(self.buffer, new_capacity);
            self.buffer = new_buf;
        }
    }

    pub fn push_str(self: *StringBuilder, s: []const u8) !void {
        const new_len = self.len + s.len;
        try self.ensureCapacity(new_len);
        std.mem.copyForwards(u8, self.buffer[self.len..new_len], s);
        self.len = new_len;
    }

    pub fn push_char(self: *StringBuilder, c: u8) !void {
        try self.ensureCapacity(self.len + 1);
        self.buffer[self.len] = c;
        self.len += 1;
    }

    pub fn toUpperInPlace(self: *StringBuilder) void {
        for (self.buffer[0..self.len]) |*c| {
            if (c.* >= 'a' and c.* <= 'z') {
                c.* = c.* - ('a' - 'A');
            }
        }
        return self;
    }

    pub fn toString(self: *StringBuilder) String {
        return String.fromSlice(self.buffer[0..self.len]);
    }
};

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    const original = String.fromSlice("hello world, hello zig!");
    const modified = try original.toUpperCase(allocator);
    std.debug.print("Modified string: {s}\n", .{modified.data});
    std.debug.print("Original string: {s}\n", .{original.data});
    var builder = try original.mutable(allocator);
    defer builder.deinit();

    try builder.replaceInPlace("hello", "hi");
    builder.toUpperInPlace();

    std.debug.print("Final modified string: {s}\n", .{builder.toString().data});
}
